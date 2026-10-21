# 9. Behavioral specification (for reimplementers)

Documents 01–08 describe *how* Expat is built. This document describes
*what an application can observe*: the contract a compatible reimplementation
must honor. Where Expat departs from the XML spec, or does something
surprising, it is marked **⚑ quirk**. Applications and Expat's own test suite
often depend on these quirks, so a drop-in replacement should reproduce them
unless you deliberately decide to differ.

The authoritative user documentation is `expat/doc/reference.html`, and the
handler contracts are in the comments in `expat/lib/expat.h`. This document
adds what those leave implicit.

---

## 9.1 Scope

| Feature | Supported? |
|---------|------------|
| XML 1.0 well-formedness | Yes. Name characters follow **XML 1.0 4th edition** (the BMP tables in `nametab.h`), not the 5th-edition ranges. Supplementary-plane characters (4-byte UTF-8, surrogate pairs) are accepted as **data** but **never as name characters**. |
| XML 1.1 | No. The version value is not checked (⚑ any `[A-Za-z0-9._-]*` is accepted), but 1.1 rules (NEL, C1 controls, extended names) are never applied. |
| Validation | No. Content models and attribute types are reported. Attribute types only affect normalization and ID detection. `#REQUIRED`/`#FIXED`, IDREF, ENTITY, and similar constraints are **not** enforced. |
| Namespaces 1.0 | Yes, opt-in (`XML_ParserCreateNS`). |
| DTD internal subset | Always processed (entity and attribute-list declarations take effect). |
| External DTD subset and parameter entities | Only with `XML_DTD`, `XML_SetParamEntityParsing` ≠ NEVER, and an external-entity handler. |
| I/O, URI resolution, catalogs | Never. The application is called through `ExternalEntityRefHandler`. |
| Encodings | Built in: UTF-8, UTF-16 (BE/LE, BOM or auto-detected), ISO-8859-1, US-ASCII. Anything else through `UnknownEncodingHandler` (8-bit ASCII-supersets, at most 4 bytes per character, BMP only). |
| API character type | `XML_Char` = UTF-8 `char` (default), or UTF-16 `unsigned short` / `wchar_t` (`XML_UNICODE[_WCHAR_T]`). |

---

## 9.2 Lifecycle and state machine

```
            XML_ParserCreate*/Reset
                    │
                    ▼
            ┌──────────────┐  XML_Parse/ParseBuffer  ┌───────────┐  isFinal && ok   ┌──────────┐
            │ INITIALIZED  │────────────────────────►│  PARSING  │─────────────────►│ FINISHED │
            └──────────────┘                         └───────────┘                  └──────────┘
                    ▲                                 │   ▲  │ XML_StopParser(FALSE)     ▲
                    │ XML_ParserReset                 │   │  └───────────────────────────┤
                    │                 StopParser(TRUE)│   │XML_ResumeParser              │
                    │                                 ▼   │                              │
                    │                              ┌───────────┐  StopParser(FALSE)      │
                    │                              │ SUSPENDED │─────────────────────────┘
                    │                              └───────────┘
          any error ──► sticky error state (status stays PARSING, every later call
                        returns XML_STATUS_ERROR with the same code) until Reset
```

| Call | INITIALIZED | PARSING | SUSPENDED | FINISHED |
|------|-------------|---------|-----------|----------|
| `XML_Parse` / `XML_ParseBuffer` | starts (salt, implicit NS context) | continues | ERROR `XML_ERROR_SUSPENDED` | ERROR `XML_ERROR_FINISHED` |
| `XML_GetBuffer` | ok | ok | NULL, `XML_ERROR_SUSPENDED` | NULL, `XML_ERROR_FINISHED` |
| `XML_StopParser(p, resumable)` | ERROR `XML_ERROR_NOT_STARTED` | → SUSPENDED / FINISHED (PE parser + resumable: `XML_ERROR_SUSPEND_PE`) | resumable: `XML_ERROR_SUSPENDED`; else → FINISHED | ERROR `XML_ERROR_FINISHED` |
| `XML_ResumeParser` | `XML_ERROR_NOT_SUSPENDED` | `XML_ERROR_NOT_SUSPENDED` | continue from saved point | `XML_ERROR_NOT_SUSPENDED` |
| `XML_SetEncoding`, `XML_UseForeignDTD`, `XML_SetParamEntityParsing`, `XML_SetHashSalt`, `XML_SetReturnNSTriplet` | allowed | refused | refused | accepted but pointless (`XML_ParserReset` reinitializes these) |

Return values: `XML_Parse`, `XML_ParseBuffer`, and `XML_ResumeParser` return
`XML_STATUS_OK` (1), `XML_STATUS_ERROR` (0), or `XML_STATUS_SUSPENDED` (2).
After an abort through `XML_StopParser(p, XML_FALSE)`, the call returns
`XML_STATUS_ERROR` with `XML_ERROR_ABORTED`.

`XML_StopParser` is effective **at the next token boundary**. The handler
that called it returns normally. Some trailing events are still delivered:
the end handler of an empty element whose start handler stopped the parser,
the end-namespace handlers of an end tag, and so on.

**Incremental guarantee:** for any split of the input into `XML_Parse` calls,
the sequence of events is the same, except that character data (and
default-handler text) may be divided differently. An incomplete token at the
end of a non-final buffer is never an error; it is kept for the next call.
Errors about incomplete input (`UNCLOSED_TOKEN`, `PARTIAL_CHAR`,
`NO_ELEMENTS`, `UNCLOSED_CDATA_SECTION`) appear only when `isFinal` is true.
A CR at the end of a non-final buffer is held back in case it is the first
half of CRLF. The same goes for `]` and `]]` at the end of content (a
possible `]]>`).

---

## 9.3 Events

### Handler fallback rule

Every construct is offered first to its **specific handler**. If there is
none and a **default handler** is set, the construct's raw text is sent to
the default handler instead: transcoded to `XML_Char`, **without**
end-of-line normalization, possibly in several pieces. Constructs that have
no specific handler (whitespace in the prolog, `<!DOCTYPE` keywords and
punctuation, the XML declaration when there is no XML-declaration handler,
and so on) always go to the default handler. The byte order mark is never
reported.

`XML_DefaultCurrent(p)`, called inside a handler, sends the current
construct's raw text to the default handler as well.

`XML_SetDefaultHandler` also **turns off expansion of internal general
entities in content** ⚑. Such references go to the skipped-entity handler
(`is_parameter_entity = 0`) or the default handler as raw `&name;`.
`XML_SetDefaultHandlerExpand` sets the default handler and leaves expansion
on. Entities in attribute values are expanded in both cases.

### Content events

| Construct | Handler call | Details |
|-----------|--------------|---------|
| Start tag `<n a="v">` | `StartElement(ud, name, atts)` | `atts` is a NULL-terminated array of name, value, name, value…. **Specified attributes first, in document order, then defaulted attributes in declaration order.** `XML_GetSpecifiedAttributeCount()` gives `2 × #specified`. `XML_GetIdAttributeIndex()` gives the index of the ID-typed attribute *if it was specified*, otherwise -1. In namespace mode, `xmlns` and `xmlns:*` attributes are **removed** from `atts`; without namespaces they are ordinary attributes. |
| End tag | `EndElement(ud, name)` | same `name` string format as the start |
| Empty element `<n/>` | `StartElement` then `EndElement` straight away | no character data in between |
| Character data | `CharacterData(ud, s, len)` | **Not NUL-terminated.** May be split anywhere between tokens, and also inside a token when transcoding fills the 1024-unit buffer. Each line break is delivered as its own call with `"\n"` (CRLF and lone CR become LF). Character references and predefined entities are delivered as their own calls with the decoded character. |
| `<![CDATA[…]]>` | `StartCdataSection`, then `CharacterData`…, then `EndCdataSection` | newlines normalized; the content is never parsed as markup |
| `<?target data?>` | `ProcessingInstruction(ud, target, data)` | `data` has leading whitespace stripped and CR/CRLF normalized to LF; NUL-terminated |
| `<!--c-->` | `Comment(ud, data)` | newlines normalized |
| `&name;` (internal) | the expanded replacement text produces normal events | elements started inside an entity must end inside it (`XML_ERROR_ASYNC_ENTITY`) |
| `&name;` (external parsed) | `ExternalEntityRefHandler(arg, context, base, systemId, publicId)` | `arg` is the parser unless `XML_SetExternalEntityRefHandlerArg` was used. Return 0 for `XML_ERROR_EXTERNAL_ENTITY_HANDLING`. With no handler: default handler, otherwise nothing. |
| `&name;` undeclared, but declarations may be missing (see 9.6) | `SkippedEntity(ud, name, 0)`, otherwise default handler | |

### Namespace events (namespace mode only)

* `StartNamespaceDecl(ud, prefix, uri)` is called for each `xmlns` attribute,
  **before** that element's `StartElement`. `prefix` is NULL for the default
  namespace. `uri` is NULL for `xmlns=""`.
* `EndNamespaceDecl(ud, prefix)` is called **after** the matching
  `EndElement`.
* **Expanded names:** `URI + sep + local`, or, with `XML_SetReturnNSTriplet(p, 1)`
  and a prefixed name, `URI + sep + local + sep + prefix`. Unprefixed
  element names are expanded only if a default namespace is in scope.
  Unprefixed **attribute** names are **never** expanded. If `sep` is `'\0'`,
  URI and local name are simply concatenated.
* The prefix `xml` is always bound to `http://www.w3.org/XML/1998/namespace`.

### Prolog and DTD events

| Construct | Handler |
|-----------|---------|
| `<?xml …?>` / text declaration | `XmlDecl(ud, version, encoding, standalone)`; `version` is NULL for a text declaration; `standalone` is -1, 0, or 1 |
| `<!DOCTYPE` | `StartDoctypeDecl(ud, name, sysid, pubid, has_internal_subset)`. It is called at `[` (has_internal_subset = 1) or at the closing `>` if there is no internal subset (= 0). `pubid` is normalized. |
| end of DOCTYPE | `EndDoctypeDecl(ud)`, after the external subset (if any) has been processed |
| `<!ENTITY>` | `EntityDecl(ud, name, is_param, value, value_len, base, systemId, publicId, notation)`. For internal entities, `value` is the replacement text, **not** NUL-terminated. For external ones, `value` is NULL. `notation` is set for unparsed entities. The obsolete `UnparsedEntityDecl` handler, if set, takes precedence for NDATA entities. |
| `<!NOTATION>` | `NotationDecl(ud, name, base, systemId, publicId)` |
| `<!ATTLIST>` | `AttlistDecl(ud, elname, attname, att_type, dflt, isrequired)`, **once per attribute**. `att_type` is `"CDATA"`, `"ID"`, `"IDREF"`, `"IDREFS"`, `"ENTITY"`, `"ENTITIES"`, `"NMTOKEN"`, `"NMTOKENS"`, `"(a\|b\|c)"`, or `"NOTATION(a\|b)"` (no spaces). `#IMPLIED`: `dflt` NULL, `isrequired` 0. `#REQUIRED`: NULL, 1. Plain default: value, 0. `#FIXED`: value, **1** ⚑. |
| `<!ELEMENT>` | `ElementDecl(ud, name, model)`. The application owns `model` (free it with `XML_FreeContentModel`). The layout is in [05 §5.3](05-dtd-and-entities.md#element-declarations-and-content-models): a root node of type EMPTY, ANY, MIXED, CHOICE, or SEQ, with `children` arrays inside the same block. |
| PI / comment in the DTD | same handlers as in content |
| `%pe;` (external, read) | `ExternalEntityRefHandler(arg, NULL, base, systemId, publicId)`; `context` is NULL for parameter entities |
| external subset | same, using the DOCTYPE's sysid/pubid (or all NULL with `XML_UseForeignDTD`) |
| `%pe;` **undeclared** (PE parsing enabled, declarations may be missing) | `SkippedEntity(ud, name, 1)`, only for references *between* declarations, never inside one. A declared external PE that is not read (PE parsing off, no handler, or the handler did not parse it) produces **no** skipped-entity event; it only stops later declarations from being processed. |
| Not standalone | `NotStandalone(ud)`, return 0 to abort with `XML_ERROR_NOT_STANDALONE` |

Declarations made after an unread parameter-entity reference in a
non-standalone document are **parsed but ignored** (no handler calls and no
effect), as the XML spec requires.

---

## 9.4 Text normalization rules

| Context | Line ends | Char refs `&#…;` | Entity refs | Whitespace |
|---------|-----------|------------------|-------------|------------|
| Content | CR, CRLF → LF | decoded | predefined decoded; internal expanded (unless the default handler disables it); external → handler | unchanged |
| CDATA section | CR, CRLF → LF | **not** recognized | not recognized | unchanged |
| Attribute value (declared CDATA or undeclared) | each CR, LF, CRLF, or TAB becomes **one space** | decoded **without** normalization (`&#10;` stays LF) | predefined decoded; internal expanded and normalized recursively; external → `XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF`; unparsed → `XML_ERROR_BINARY_ENTITY_REF` | as described |
| Attribute value (declared non-CDATA) | as above, then runs of spaces collapsed and leading/trailing spaces removed | decoded; a `&#32;` takes part in collapsing (as XML 1.0 §3.3.3 requires), but `&#9;`/`&#10;`/`&#13;` stay as literal characters | as CDATA | collapsed |
| Entity value (`<!ENTITY e "…">`) | CR, CRLF → LF | **decoded at declaration time** | general entity refs stored **literally** (expanded at use); PE refs expanded (external DTD / PE text only) | unchanged |
| PI data, comment | CR, CRLF → LF | not recognized | not recognized | leading whitespace after the PI target removed |
| Public IDs | — | — | — | runs of whitespace → one space, trimmed |
| Default-handler text | **not** normalized | raw | raw | raw |

Whether an attribute is CDATA is decided by the `ATTLIST` declarations seen
**so far** for that element. An undeclared attribute is CDATA.

⚑ In an attribute value, a reference to an undeclared entity that is
allowed, because declarations may be missing, is **silently dropped**. No
event is generated and nothing is inserted.

⚑ `<` is not allowed in an attribute value, and that includes the
replacement text of an entity referenced from one. It gives
`XML_ERROR_INVALID_TOKEN`.

---

## 9.5 Well-formedness: what is checked, and which error

Below, "invalid token" means `XML_ERROR_INVALID_TOKEN`, and the location
reported points at the offending character.

### Characters and encoding

* Allowed characters: `#x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] |
  [#x10000-#x10FFFF]`. Anything else, including C0 controls, U+FFFE, U+FFFF,
  and lone surrogates, is an invalid token.
* Malformed UTF-8 (overlong forms, encoded surrogates, values above U+10FFFF,
  bad continuation bytes, lead bytes `F5`–`FF`) is an invalid token.
* Truncated multibyte sequence at the end of input: `XML_ERROR_PARTIAL_CHAR`.
* Character reference to a disallowed character, a surrogate, or a value
  ≥ 0x110000: `XML_ERROR_BAD_CHAR_REF`. Only lowercase `x` is accepted in
  `&#x…;`.
* Declared encoding that conflicts with the detected one (different
  `minBytesPerChar`, or the other UTF-16 byte order):
  `XML_ERROR_INCORRECT_ENCODING`.
* Unknown encoding name, and no handler accepts it:
  `XML_ERROR_UNKNOWN_ENCODING`.

### Structure

| Situation | Error |
|-----------|-------|
| No root element (empty input, or only a prolog) | `XML_ERROR_NO_ELEMENTS` |
| Anything other than whitespace, comments, or PIs after the root | `XML_ERROR_JUNK_AFTER_DOC_ELEMENT` |
| `</b>` that does not match the open element (raw bytes are compared) | `XML_ERROR_TAG_MISMATCH` |
| Input ends inside a token (`<r`, `<!-- …`, `&am`, an unterminated attribute value) | `XML_ERROR_UNCLOSED_TOKEN` |
| Input ends with elements still open (`<r>text`) | ⚑ `XML_ERROR_NO_ELEMENTS` ("no element found"), the same code as an empty document (`doContent`, `XML_TOK_NONE` with `startTagLevel == 0`) |
| Input ends inside `<![CDATA[` | `XML_ERROR_UNCLOSED_CDATA_SECTION` |
| `]]>` in content | invalid token |
| `--` inside a comment (not followed by `>`) | invalid token |
| PI target `xml` in any case mix other than exactly `xml`, e.g. `XmL` | invalid token (reserved) |
| `<?xml …?>` anywhere except the very first bytes (after an optional BOM) | `XML_ERROR_MISPLACED_XML_PI` |
| Malformed XML or text declaration (pseudo-attribute order `version`, `encoding`, `standalone`; quoting; `encoding` must start with a letter; `standalone` ∈ {yes, no}; a text declaration *requires* `encoding` and forbids `standalone`; values limited to `[A-Za-z0-9._-]`) | `XML_ERROR_XML_DECL` / `XML_ERROR_TEXT_DECL` |
| Duplicate attribute (same QName, or, with namespaces, same expanded name) | `XML_ERROR_DUPLICATE_ATTRIBUTE` |
| Missing whitespace between attributes, missing `=`, unquoted value, `<` in a value | invalid token |
| Prolog grammar violation (bad DOCTYPE/ENTITY/ATTLIST/ELEMENT/NOTATION syntax, mixing `,` and `\|` in one group) | `XML_ERROR_SYNTAX` |
| Unclosed `<![IGNORE[` at end of input | `XML_ERROR_SYNTAX` |
| Conditional section (`<![INCLUDE[` / `<![IGNORE[`) in the **internal** subset | `XML_ERROR_SYNTAX` (allowed only in the external subset or external PEs) |
| `(#PCDATA\|a)` without the required `*` | `XML_ERROR_SYNTAX` |
| Invalid public ID character (allowed: `a-zA-Z0-9 -'()+,./:=?;!*#@$_%` and CR/LF; TAB is not allowed) | `XML_ERROR_PUBLICID` |

### Entities

| Situation | Error |
|-----------|-------|
| Reference to an undeclared entity when all declarations are known (no PE references or external subset, or `standalone="yes"`) | `XML_ERROR_UNDEFINED_ENTITY` |
| Same condition, entity declared only in an external subset or PE | `XML_ERROR_ENTITY_DECLARED_IN_PE` |
| Entity references itself directly or indirectly | `XML_ERROR_RECURSIVE_ENTITY_REF` |
| `&unparsed;` | `XML_ERROR_BINARY_ENTITY_REF` |
| External entity in an attribute value | `XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF` |
| Entity text not balanced (an element starts in the entity and ends outside, or the reverse) | `XML_ERROR_ASYNC_ENTITY` |
| `%pe;` inside a declaration in the internal subset | `XML_ERROR_PARAM_ENTITY_REF` |
| Parameter entity text that does not contain complete declarations | `XML_ERROR_INCOMPLETE_PE` |
| External-entity handler returned 0 | `XML_ERROR_EXTERNAL_ENTITY_HANDLING` |
| Amplification limit exceeded | `XML_ERROR_AMPLIFICATION_LIMIT_BREACH` |

### Namespaces (namespace mode only)

| Situation | Error |
|-----------|-------|
| Prefix on an element or attribute with no binding in scope | `XML_ERROR_UNBOUND_PREFIX` |
| `xmlns:p=""` | `XML_ERROR_UNDECLARING_PREFIX` |
| Binding `xmlns:xmlns` | `XML_ERROR_RESERVED_PREFIX_XMLNS` |
| Binding `xml` to any other URI | `XML_ERROR_RESERVED_PREFIX_XML` |
| Binding any prefix (or the default namespace) to the `xml` or `xmlns` namespace URI | `XML_ERROR_RESERVED_NAMESPACE_URI` |
| A URI containing the separator character when the separator is not an RFC 3986 URI character | `XML_ERROR_SYNTAX` |
| More than one `:` in a QName, or `:` not followed by a name-start character | invalid token |

### Not checked (non-validating)

* Element content against declared models, `#REQUIRED` or `#FIXED`
  attributes, attribute value types (ID uniqueness, IDREF targets,
  enumerations), notation declarations for NDATA entities.
* The version number value.
* Unused or undeclared notations, duplicate ELEMENT declarations.

---

## 9.6 Entity and DTD rules in detail

* **First declaration wins** for entities and for each (element, attribute)
  pair. Later duplicates are parsed and ignored. Declarations of the five
  predefined entities are ignored.
* **"Declarations may be missing"** is
  `dtd->hasParamEntityRefs && !dtd->standalone`. `hasParamEntityRefs` becomes
  true when there is any PE reference, a DOCTYPE with an external ID, or
  `XML_UseForeignDTD`. In that case an undeclared general entity in content
  is reported as skipped instead of being an error.
* **Parameter-entity parsing modes:** `NEVER` (default),
  `UNLESS_STANDALONE` (becomes NEVER if `standalone="yes"`), `ALWAYS`.
  Even with parsing enabled, nothing is read unless an external-entity
  handler is set.
* **External subset timing:** it is requested when the DOCTYPE's closing
  `>` is reached, so after the internal subset. Declarations in the internal
  subset therefore come first and win.
* **External general entities** are parsed by a *child parser* that starts
  with a copy of the DTD and the current namespace bindings. They are parsed
  as content and may start with a text declaration.
* **External parameter entities and the external subset** are parsed by a
  child parser that *shares* the DTD. Inside them, conditional sections
  (`INCLUDE`/`IGNORE`) and PE references inside declarations are allowed.

---

## 9.7 Position and error reporting

* `XML_GetErrorCode` returns the last error. `XML_ErrorString(code)` returns
  a fixed English message: `NULL` for `XML_ERROR_NONE`, and the strings at
  `xmlparse.c:2872` for the rest (e.g. `"not well-formed (invalid token)"`).
  Those strings are part of the de-facto interface, since some applications
  compare them.
* `XML_GetCurrentLineNumber` is **1-based**. `XML_GetCurrentColumnNumber` is
  **0-based** and counts **characters**, not bytes (a surrogate pair or a
  multibyte UTF-8 sequence counts once). CR, LF, and CRLF each end one line.
* `XML_GetCurrentByteIndex` is the byte offset in the whole input stream
  (fed to this parser) of the start of the current event or error, or -1.
  `XML_GetCurrentByteCount` is the byte length of the current event (0 if
  there is none).
* Inside a handler, the position is the start of the construct that caused
  the event. For events from internal entity text it is the position of the
  **reference** in the document ⚑. After an error, it is the offending
  character.
* ⚑ Not every error moves the location to the offending character. For
  example, a duplicate *expanded* attribute name in namespace mode reports
  the location of the start of the tag's token (column 0 for `<r …` at the
  start of a line). An invalid PI target like `XmL` reports the character
  just after the target. Tests that check error positions pin these down.
* `XML_GetInputContext(p, &offset, &size)` returns the internal buffer and the
  offset of the current event in it. Up to `XML_CONTEXT_BYTES` (1024) bytes
  before the event are kept. It returns NULL when `XML_CONTEXT_BYTES == 0`.

---

## 9.8 Encoding resolution order

1. The **protocol encoding** passed to `XML_ParserCreate`/`XML_SetEncoding`
   or to the child-parser creator, if any. Names are case-insensitive:
   `ISO-8859-1`, `US-ASCII`, `UTF-8`, `UTF-16`, `UTF-16BE`, `UTF-16LE`.
   Other names go to the unknown-encoding handler at the start of parsing.
2. Otherwise **autodetection** from the first bytes: BOM, or the
   `00 3C` / `3C 00` patterns (see [02 §2.7](02-tokenizer.md#27-encoding-detection)).
   The default is UTF-8.
3. The **encoding declaration** in `<?xml … encoding="…"?>` is used **only if
   there was no protocol encoding**. A known name must match the detected
   width and byte order. An unknown name goes to the unknown-encoding
   handler.

A protocol encoding of `ISO-8859-1` or UTF-16 turns off some BOM sniffing for
**external parsed general entities** (tokenizer content state), so BOM-like
bytes can be read as data. Document entities and external PEs are always
sniffed.

---

## 9.9 Memory and limits

* Every allocation goes through the memory suite given to
  `XML_ParserCreate_MM`, or through malloc/realloc/free.
* Defaults the application can observe: entity amplification at most
  **100×** once output exceeds **8 MiB**, and heap amplification at most
  **100×** once allocations exceed **64 MiB**. Both are relative to the
  number of direct input bytes. Allocation refusals show up as
  `XML_ERROR_NO_MEMORY`. The formulas are in
  [07](07-memory-and-security.md).
* Hash salt: random per root parser unless `XML_SetHashSalt` is used. It
  affects only performance, never results, except for the order of entries
  in the `context` string given to external-entity handlers.
* String arguments to handlers are valid only for the duration of the call.

---

## 9.10 Build-variant matrix to support

A reimplementation intended as a drop-in replacement should support at least
the default configuration: `XML_DTD`, `XML_NS`, `XML_GE=1`,
`XML_CONTEXT_BYTES=1024`, and `char` (UTF-8) `XML_Char`. Other variants
change observable behavior as follows:

| Variant | Observable change |
|---------|-------------------|
| `XML_UNICODE` | all `XML_Char` strings are UTF-16 code units; `XML_ErrorString` and `XML_ExpatVersion` stay `XML_LChar` (narrow unless `XML_UNICODE_WCHAR_T`) |
| no `XML_DTD` | `XML_UseForeignDTD` → `XML_ERROR_FEATURE_REQUIRES_XML_DTD`; `XML_SetParamEntityParsing` only accepts NEVER; `%pe;` is never read; no conditional sections |
| `XML_GE=0` | internal general entities are not expanded: their value is reported and used as the literal text `&name;`; no amplification protection |
| `XML_CONTEXT_BYTES=0` | `XML_GetInputContext` returns NULL |
| `XML_ATTR_INFO` | `XML_GetAttributeInfo` available |
| `XML_LARGE_SIZE` | `XML_Index`/`XML_Size` are 64-bit |

`XML_GetFeatureList()` reports the variant. Tools such as `xmlwf -v` print it.

---

## 9.11 Conformance testing resources in this repository

| Resource | What it gives you |
|----------|-------------------|
| `expat/tests/runtests.c` and `basic_tests.c`, `ns_tests.c`, `misc_tests.c`, `alloc_tests.c`, `nsalloc_tests.c`, `acc_tests.c` (~12k lines) | Unit and regression tests through the public API. The alloc tests inject allocation failures at every point. The acc tests check amplification byte counts. |
| `expat/tests/xmltest.sh` + `xmlts.zip` + `xmltest.log.expected` | Runs the W3C XML Conformance Test Suite through `xmlwf` and compares the output with the expected log, **including Expat's known deviations** |
| `expat/xmlwf/` | Reference command-line tool: well-formedness checker and canonical-ish output writer. Handy for differential testing. |
| `expat/fuzz/` | libFuzzer harnesses (`XML_Parse`, `XML_ParseBuffer`, and a structure-aware protobuf fuzzer) |
| `expat/doc/reference.html` | The official API reference |

A practical strategy is to build your implementation behind the same
`expat.h` ABI and link it against `runtests` and `xmlwf`. That runs the
existing suites unchanged. Tests that inspect internals
(`testingAccountingGetCountBytes*`, `g_bytesScanned`,
`expat_malloc`/`expat_free` under `XML_TESTING`) need small shims.
