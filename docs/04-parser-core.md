# 4. The parser core: `xmlparse.c`

`xmlparse.c` holds all of Expat's state and all of its public API. This
document covers the parser object, input handling, the processor mechanism,
and the content (element) side of parsing. The DTD and entities are in
[05](05-dtd-and-entities.md); the containers are in
[06](06-data-structures.md).

## 4.1 Map of `xmlparse.c`

| Lines (approx.) | Section |
|----------------:|---------|
| 1–200 | Config sanity `#error`s, entropy-source checks, `XML_UNICODE` switches (`ICHAR`, `XmlConvert`, `MUST_CONVERT`), `XML_T`/`XML_L` literals |
| 200–470 | Internal types: `HASH_TABLE`, `BINDING`, `PREFIX`, `TAG_NAME`, `TAG`, `ENTITY`, `CONTENT_SCAFFOLD`, `BLOCK`/`STRING_POOL`, `ATTRIBUTE_ID`, `DEFAULT_ATTRIBUTE`, `NS_ATT`, `ELEMENT_TYPE`, `DTD`, `OPEN_INTERNAL_ENTITY`, `ACCOUNTING`, `MALLOC_TRACKER`, `ENTITY_STATS` |
| 470–660 | `Processor` typedef, forward declarations, `pool*` macros |
| 663–790 | **`struct XML_ParserStruct`**, `MALLOC`/`REALLOC`/`FREE` macros |
| 790–1010 | Allocation tracker: `expat_malloc`, `expat_realloc`, `expat_free` |
| 1010–1340 | Creation helpers, entropy and hash salt, **`callProcessor`**, `startParsing` |
| 1340–2020 | `parserCreate`, `parserInit`, `XML_ParserReset`, `XML_SetEncoding`, **`XML_ExternalEntityParserCreate`**, `XML_ParserFree` |
| 2020–2340 | Handler setters and simple configuration API |
| 2340–2860 | **`XML_Parse`, `XML_ParseBuffer`, `XML_GetBuffer`**, `XML_StopParser`, `XML_ResumeParser`, error/position queries |
| 2860–3125 | `XML_ErrorString`, version, feature list, security-limit setters |
| 3125–3830 | `storeRawNames`, content processors, external-entity init processors, **`doContent`** |
| 3830–4575 | **`storeAtts`**, `addBinding` (namespaces) |
| 4575–4830 | CDATA and IGNORE sections |
| 4830–5195 | Encoding initialization, XML declaration, unknown encodings, prolog and PE processors |
| 5195–6270 | **`doProlog`** |
| 6270–6500 | `epilogProcessor`, **`processEntity`, `internalEntityProcessor`**, `errorProcessor` |
| 6500–7050 | Attribute value and entity value storage/expansion |
| 7050–7500 | PI/comment/default reporting, `defineAttribute`, `getAttributeId`, `getContext`/`setContext` |
| 7500–7800 | DTD create/reset/destroy/copy |
| 7800–8220 | Hash table and string pool |
| 8220–8460 | Content-model scaffolding and `build_model` |
| 8460–9224 | Amplification accounting, entity tracking, debug helpers |

## 4.2 `struct XML_ParserStruct`

`XML_Parser` is an opaque pointer to `struct XML_ParserStruct`
(`xmlparse.c:663`). Its fields fall into these groups:

| Group | Fields | Notes |
|-------|--------|-------|
| **User context** | `m_userData`, `m_handlerArg` | `m_userData` **must be first**: the public macro `XML_GetUserData(parser)` is defined in `expat.h` as `(*(void **)(parser))`, so this is part of the ABI. `m_handlerArg` is passed to handlers; it equals `m_userData` unless `XML_UseParserAsHandlerArg` sets it to the parser. |
| **Input buffer** | `m_buffer`, `m_bufferPtr`, `m_bufferEnd`, `m_bufferLim`, `m_parseEndByteIndex`, `m_parseEndPtr`, `m_partialTokenBytesBefore`, `m_reparseDeferralEnabled`, `m_lastBufferRequestSize` | See 4.4 |
| **Scratch for char data** | `m_dataBuf`, `m_dataBufEnd` | `INIT_DATA_BUF_SIZE` (1024) `XML_Char`s, used when text must be converted before a handler call |
| **Handlers** | `m_startElementHandler` … `m_xmlDeclHandler`, `m_externalEntityRefHandlerArg`, `m_unknownEncodingHandler(+Data)` | about 25 function pointers |
| **Encoding** | `m_encoding`, `m_initEncoding`, `m_internalEncoding`, `m_protocolEncodingName`, `m_unknownEncoding{Mem,Data,Release}`, `m_unknownEncodingHandlerData` | `m_encoding` starts out pointing at `m_initEncoding.initEnc`, the detecting scanners |
| **Namespaces** | `m_ns`, `m_ns_triplets`, `m_namespaceSeparator`, `m_inheritedBindings`, `m_freeBindingList`, `m_nsAtts`, `m_nsAttsVersion`, `m_nsAttsPower` | |
| **Control** | `m_prologState`, `m_processor`, `m_errorCode`, `m_parsingStatus`, `m_reenter` | `m_processor` is the central dispatch pointer |
| **Event location** | `m_eventPtr`, `m_eventEndPtr`, `m_positionPtr`, `m_position` | the span of the current token (for error reporting and `XML_DefaultCurrent`), and lazy line/column tracking |
| **Entities in progress** | `m_openInternalEntities`/`m_freeInternalEntities`, `m_openAttributeEntities`/`m_freeAttributeEntities`, `m_openValueEntities`/`m_freeValueEntities`, `m_defaultExpandInternalEntities` | three explicit stacks plus free lists, see [05](05-dtd-and-entities.md#54-expanding-internal-entities-without-recursion) |
| **Element stack** | `m_tagLevel`, `m_tagStack`, `m_freeTagList` | |
| **Attributes** | `m_atts`, `m_attsSize`, `m_nSpecifiedAtts`, `m_idAttIndex`, `m_attInfo` | `m_atts` is reused and also serves as the array passed to the start handler (see 4.8) |
| **DTD declaration registers** | `m_declEntity`, `m_declElementType`, `m_declAttributeId`, `m_declAttributeType`, `m_declAttributeIsCdata/IsId`, `m_declNotationName/PublicId`, `m_doctypeName/Sysid/Pubid` | state carried between roles inside one declaration |
| **DTD** | `m_dtd`, `m_curBase` | `m_dtd` may be shared with the parent parser (external PE parsers) |
| **Temporary storage** | `m_tempPool`, `m_temp2Pool`, `m_groupConnector`, `m_groupSize` | `m_tempPool` holds per-start-tag attribute values and other short-lived strings |
| **Parent link** | `m_parentParser` | non-NULL for external entity parsers; used to find the root parser |
| **DTD options** (`XML_DTD`) | `m_isParamEntity`, `m_useForeignDTD`, `m_paramEntityParsing` | |
| **Security** | `m_hash_secret_salt`, `m_accounting`, `m_alloc_tracker`, `m_entity_stats` | the latter three exist only with `XML_GE=1` and are meaningful only in the root parser |
| **Memory** | `m_mem` | a `const XML_Memory_Handling_Suite` (malloc/realloc/free), written once through a cast in `parserCreate` |

## 4.3 Lifecycle

```
XML_ParserCreate(enc)          ┐
XML_ParserCreateNS(enc, sep)   ├─► XML_ParserCreate_MM ─► parserCreate(enc, mem, sep, dtd=NULL, parent=NULL)
XML_ParserCreate_MM(...)       ┘        │  allocate struct (+ size header when XML_GE)
                                        │  allocate m_atts, m_dataBuf, dtdCreate()
                                        │  poolInit temp pools, parserInit()
                                        │  m_internalEncoding = (NS ? ...NS : ...)()
                                        ▼
                                 state XML_INITIALIZED, m_processor = prologInitProcessor
   first XML_Parse / XML_ParseBuffer
        └─► startParsing(): generate hash salt if unset; if m_ns: setContext("xml=http://www.w3.org/XML/1998/namespace")
                                        ▼
                                 state XML_PARSING ─► … ─► XML_FINISHED (isFinal) / error
XML_ParserReset(p, enc)  — root parser only: move tags/bindings/open entities to free lists,
                           clear pools, parserInit(), dtdReset(). Keeps allocations for reuse.
XML_ParserFree(p)        — free everything (the DTD only if not shared with a parent)
```

`parserInit` (`xmlparse.c:1539`) resets everything that `XML_ParserReset`
must reset: handlers, buffer pointers, prolog state, position, open-entity
lists, accounting limits (reset to their defaults, with debug levels read
from the environment), and so on. `parserCreate` does the one-time
allocations.

Some settings must be made before parsing starts and are refused afterward
(`parserBusy()`): `XML_SetEncoding`, `XML_UseForeignDTD`,
`XML_SetReturnNSTriplet`, `XML_SetParamEntityParsing`, and `XML_SetHashSalt`
(which checks the *root* parser). `XML_SetReparseDeferralEnabled` can be
called at any time. The amplification-limit setters are refused only on child
parsers.

## 4.4 Input buffering

Applications feed input in one of two ways:

1. **`XML_Parse(p, s, len, isFinal)`**: Expat copies `s` into its own buffer
   (through `XML_GetBuffer`) and then calls `XML_ParseBuffer`.
2. **`XML_GetBuffer(p, len)` + `XML_ParseBuffer(p, len, isFinal)`**: the
   application writes straight into Expat's buffer, which saves one copy.

The buffer layout is described in the struct comment (`xmlparse.c:669`):

```
   m_buffer <= m_bufferPtr <= m_bufferEnd  <= m_bufferLim
   |           |              |               |
   <--parsed-->|              |               |
               <---parsing--->|               |
                              <--unoccupied-->|
   <---------total-malloced/realloced-------->|
```

* `m_bufferPtr` is the first byte the processors have **not** consumed yet.
  Everything before it has been fully handled, except that up to
  `XML_CONTEXT_BYTES` of it is kept for `XML_GetInputContext`.
* `m_bufferEnd` is the end of the valid data.

`XML_GetBuffer` (`xmlparse.c:2523`) works like this:

* If `len` fits in `[m_bufferEnd, m_bufferLim)`, it returns `m_bufferEnd`.
* Otherwise it computes `neededSize = unparsed + len + keep`, where
  `keep = min(XML_CONTEXT_BYTES, parsed)`.
  * If the existing allocation is big enough, it `memmove`s the unparsed tail
    (plus `keep` bytes of context) to the front.
  * If not, it doubles from `INIT_BUFFER_SIZE` (1024) until big enough,
    allocates a new buffer, copies, and frees the old one.
* All size math is done in `unsigned` and checked, so it never overflows
  `int`.
* **`m_buffer` is allocated with plain `m_mem.malloc_fcn`, not `MALLOC()`.**
  The input buffer is deliberately left out of the allocation tracker. The
  application already controls how much input it sends, so limiting it is the
  application's job (see the comment at `xmlparse.c:2604`).

`XML_ParseBuffer` (`xmlparse.c:2452`) advances `m_bufferEnd` by `len`, records
`finalBuffer = isFinal`, runs `callProcessor(parser, m_bufferPtr, m_bufferEnd,
&m_bufferPtr)`, and updates the position. Unconsumed bytes (a partial token)
stay in `[m_bufferPtr, m_bufferEnd)` for the next call.

**Zero-copy fast path.** When `XML_CONTEXT_BYTES == 0` and the internal buffer
is empty, `XML_Parse` runs the processor directly on the caller's memory and
copies only the leftover partial token into `m_buffer` (`xmlparse.c:2364`).

**Pointers into the buffer are temporary.** Anything that must survive a
buffer move is copied. The main case is the raw names of open elements
(`storeRawNames`, 4.7). Strings passed to handlers are valid only during the
callback.

## 4.5 Processors

```c
typedef enum XML_Error PTRCALL Processor(XML_Parser parser,
                                         const char *start, const char *end,
                                         const char **endPtr);
```

A processor consumes as much of `[start, end)` as it can, sets `*endPtr` to
where it stopped, and returns an error code. `m_processor` names the processor
for the next chunk, so changing what mode the parser is in means assigning
`m_processor`. The processors and their transitions:

```
 Document entity
 ───────────────
 prologInitProcessor ─initializeEncoding─► prologProcessor ──(doProlog)──┐
                                              ▲    │ XML_ROLE_IGNORE_SECT│ XML_ROLE_INSTANCE_START
                                              │    ▼                     ▼
                                   ignoreSectionProcessor         contentProcessor ──(doContent)──┐
                                                                     │ ▲   │ <![CDATA[ split     │ root end tag
                                                                     │ │   ▼ across buffers      ▼
                                                                     │ └ cdataSectionProcessor  epilogProcessor
                                                                     │
          any processor ──&entity;/%pe; (internal)──► internalEntityProcessor ──when stack empty──►
                                                      back to contentProcessor / prologProcessor
          any error ─────────────────────────────────► errorProcessor (returns m_errorCode forever)

 External general entity parser (XML_ExternalEntityParserCreate with context != NULL)
 ─────────────────────────────────────────────────────────────────────────────────
 externalEntityInitProcessor ─encoding─► …InitProcessor2 ─BOM─► …InitProcessor3 ─<?xml?>─►
 externalEntityContentProcessor (doContent with startTagLevel = 1)

 External parameter entity / external subset parser (context == NULL, XML_DTD)
 ─────────────────────────────────────────────────────────────────────────────
 externalParEntInitProcessor ─┬─► externalParEntProcessor ─► prologProcessor (doProlog)
                              └─(inEntityValue)─► entityValueInitProcessor ─► entityValueProcessor
```

Most processors are thin wrappers. The real work is in four `do*` functions
that take an explicit `enc`, so they can run over either the document bytes
(`m_encoding`) or internal entity text (`m_internalEncoding`):

* `doProlog(parser, enc, s, end, tok, next, nextPtr, haveMore, allowClosingDoctype, account)`
* `doContent(parser, startTagLevel, enc, s, end, nextPtr, haveMore, account)`
* `doCdataSection(...)`, `doIgnoreSection(...)`

`haveMore` is `!finalBuffer` for document input and `XML_FALSE` for entity
text, because entity text is always complete. It decides whether a
`PARTIAL`/`TRAILING_CR`/`NONE` token means "wait for more" or "error / end".

Each `do*` function sets up `eventPP`/`eventEndPP` to point either at
`m_eventPtr`/`m_eventEndPtr` (document bytes) or at the current open
entity's `internalEventPtr`/`internalEventEndPtr` (entity text). Error
locations and `XML_DefaultCurrent` therefore always refer to the right
buffer.

## 4.6 `callProcessor`: the driver loop

Every entry point (`XML_Parse`, `XML_ParseBuffer`, `XML_ResumeParser`)
reaches the processors only through `callProcessor` (`xmlparse.c:1257`):

```c
static enum XML_Error
callProcessor(XML_Parser parser, const char *start, const char *end,
              const char **endPtr) {
  // 1) Reparse deferral (see doc 07): if the last attempt consumed nothing
  //    and we don't yet have at least 2x as many bytes, return without
  //    scanning. Exception: when close to forcing a buffer reallocation.
  ...
  // 2) Run processors in a loop instead of letting them recurse.
  *endPtr = start;
  while (1) {
    ret = parser->m_processor(parser, *endPtr, end, endPtr);
    if (parser->m_parsingStatus.parsing != XML_PARSING)
      parser->m_reenter = XML_FALSE;     // suspend/abort wins over re-enter
    if (! parser->m_reenter)
      break;
    parser->m_reenter = XML_FALSE;
    if (ret != XML_ERROR_NONE)
      return ret;
  }
  // 3) Remember whether we made progress (for deferral next time).
  ...
}
```

The **re-enter flag** is how Expat avoids recursion. When `doContent` meets
`&e;` for an internal entity, it does not call itself on the entity's text.
`processEntity` pushes an `OPEN_INTERNAL_ENTITY`, sets
`m_processor = internalEntityProcessor`, and calls `triggerReenter()`.
`doContent` sees `m_reenter` after the token, returns at once with `*nextPtr`
just past the reference, and `callProcessor` loops. The next iteration runs
`internalEntityProcessor`, which tokenizes the entity's replacement text. If
that text contains another reference, the same thing happens one level
deeper. The C stack depth stays constant no matter how deeply entities nest.
Details are in [05 §5.4](05-dtd-and-entities.md#54-expanding-internal-entities-without-recursion).

## 4.7 Parsing status, suspend, resume, abort

`m_parsingStatus` is `{ enum XML_Parsing parsing; XML_Bool finalBuffer; }`
with states `XML_INITIALIZED → XML_PARSING → XML_FINISHED`, and
`XML_SUSPENDED` as a side state.

* `XML_StopParser(p, resumable)` may be called **from inside a handler**. It
  only changes `parsing` to `XML_SUSPENDED` (resumable) or `XML_FINISHED`
  (abort). Suspension is refused while parsing an external PE
  (`XML_ERROR_SUSPEND_PE`).
* Every `do*` loop checks the status **after each token**:
  * `SUSPENDED`: set `*nextPtr = next` and return `XML_ERROR_NONE`.
    `m_bufferPtr` stays at that point.
  * `FINISHED`: return `XML_ERROR_ABORTED`.
  * `PARSING` with `m_reenter`: return to `callProcessor` (see above).
* `XML_ResumeParser` calls `callProcessor(p, m_bufferPtr, m_parseEndPtr, …)`
  again with the same `m_processor`.
* While suspended, `XML_Parse`, `XML_ParseBuffer`, and `XML_GetBuffer`
  return `XML_ERROR_SUSPENDED`. Internally `XML_Parse` briefly sets the
  status back to `PARSING` around its own `XML_GetBuffer` call so it can save
  leftover bytes.

Because this works per token, resumable state must live in the parser
struct and not on the C stack. That requirement is one reason for the
explicit entity stacks and the `entity->processed` offsets.

**Errors are sticky.** On any error the entry point sets
`m_processor = errorProcessor`, which just returns `m_errorCode`. Further
calls fail with the same error until `XML_ParserReset`.

## 4.8 `doContent`: element content

`doContent` (`xmlparse.c:3312`) loops over `XmlContentTok` tokens. With
`XML_GE`, each token's bytes are first checked by `accountingDiffTolerated`
(amplification protection). The main cases:

| Token | Action |
|-------|--------|
| `DATA_CHARS` | Call the character-data handler. Pass the input bytes directly if `!MUST_CONVERT`; otherwise convert in chunks through `m_dataBuf`, so the handler may be called several times for one token. Without a character-data handler, use the default handler. |
| `DATA_NEWLINE`, `TRAILING_CR` | Pass a single `'\n'`. This is where CR/CRLF become LF (XML end-of-line handling). `TRAILING_CR` waits for more input if `haveMore`. |
| `CHAR_REF` | `XmlCharRefNumber`, then `XmlEncode` into a small buffer, then the character-data handler. |
| `ENTITY_REF` | Predefined entities (`&lt;` etc.) become a single character. Otherwise look the name up in `dtd->generalEntities`, apply the WFC/VC rules about undeclared entities (it depends on `standalone` and `hasParamEntityRefs`), detect recursion (`entity->open`) and unparsed entities (`notation`), then either `processEntity()` (internal entity), call `m_externalEntityRefHandler` with `getContext()` (external entity), or call the skipped-entity or default handler. |
| `START_TAG_*` | Get a `TAG` from `m_freeTagList` or allocate one. Push it on `m_tagStack`. Convert the name into `tag->buf`, growing it as needed. `storeAtts()`. Call the start handler with `tag->name.str` and `m_atts`. `poolClear(&m_tempPool)`. |
| `EMPTY_ELEMENT_*` | Like a start tag, but the name goes into `m_tempPool` and no `TAG` is pushed. Call the start handler and then the end handler at once, then free the bindings. |
| `END_TAG` | Compare the raw bytes with `m_tagStack->rawName` (`XML_ERROR_TAG_MISMATCH` if different). Pop to the free list. In namespace mode, rebuild the expanded name in the binding's URI buffer (4.10). Call the end handler. Unwind the tag's namespace bindings (end-namespace handler, restore `prefix->binding = prevPrefixBinding`). |
| `CDATA_SECT_OPEN` | Start-CDATA handler, then `doCdataSection`. If the section continues past the end of the buffer, switch to `cdataSectionProcessor`. |
| `PI`, `COMMENT` | `reportProcessingInstruction` / `reportComment`. These copy into `m_tempPool` and normalize newlines. |
| `XML_DECL` | `XML_ERROR_MISPLACED_XML_PI`, since an XML declaration is only allowed at the very start. |
| `NONE`, `PARTIAL`, `PARTIAL_CHAR`, `TRAILING_RSQB` | End of input: wait if `haveMore`, otherwise report the right error or finish. |

When `m_tagLevel` drops back to 0 after a root end tag or a root empty
element, `doContent` changes to `epilogProcessor`. It calls it directly, or
just sets `m_processor` if the parser is suspended or re-entering.
`epilogProcessor` accepts only whitespace, comments, and PIs; anything else
is `XML_ERROR_JUNK_AFTER_DOC_ELEMENT`.

`startTagLevel` is the tag depth at which this `doContent` invocation
started: 0 for the document, 1 for an external entity, and the current
`m_tagLevel` for internal entity text. An end tag at that level, or reaching
the end of entity text at a different level, gives `XML_ERROR_ASYNC_ENTITY`.
This enforces the rule that an entity's replacement text must be balanced.

### Raw names and `storeRawNames`

A `TAG` stores its name twice:

* `rawName` / `rawNameLength`: bytes in the **document encoding**, used for a
  fast `memcmp` against the end tag; and
* `name` (`TAG_NAME`): the converted `XML_Char` name, plus namespace parts.

At first `rawName` points straight into the input buffer. Before any
content processor returns, `storeRawNames` (`xmlparse.c:3128`) walks up the
tag stack and copies each `rawName` into the end of that tag's own `buf`,
growing it if needed. It stops at the first tag that was already copied: the
stack is LIFO, so everything below has been copied too. After that it is safe
for `XML_GetBuffer` to move or free the input buffer.

## 4.9 `storeAtts`: attributes

`storeAtts` (`xmlparse.c:3827`) runs for **every** start tag and
empty-element tag, with or without attributes. It also does element-name
namespace expansion and attribute defaulting. Its header comment lists
what it does: normalize attributes, check them for well-formedness, build
namespace-aware attribute names, build the list for the start handler, apply
default attributes, process namespace declarations, and build the
namespace-aware element name. Step by step:

1. **Find or create the `ELEMENT_TYPE`** in `dtd->elementTypes`. With
   namespaces on, `setElementTypePrefix` records the element's `PREFIX`.
2. **Get the raw attributes** with `XmlGetAttributes(enc, attStr,
   m_attsSize, m_atts)`. If there are more than fit (counting the element's
   defaults too), grow `m_atts` (and `m_attInfo`) and call it again.
3. **For each attribute:**
   * `getAttributeId()` interns the name in `dtd->attributeIds`. The stored
     name has **one hidden `XML_Char` before it** (`name[-1]`) that serves as
     a per-attribute flag. See [06 §6.6](06-data-structures.md#66-the-name-1-flag-byte-on-attribute-ids).
   * **Duplicate check:** if `name[-1]` is already set, report
     `XML_ERROR_DUPLICATE_ATTRIBUTE`. Otherwise set it to 1. This finds
     duplicates in O(n) without a separate set.
   * **Value:** if the tokenizer marked it `normalized`, copy it directly
     with `poolStoreString`. Otherwise `storeAttributeValue` decodes
     references, expands entities, and normalizes whitespace (also collapsing
     spaces if the declared type is not CDATA, which it learns from
     `elementType->defaultAtts[]`).
   * **Namespaces:** for `xmlns` / `xmlns:p`, `addBinding()` and remove the
     attribute from the list (`--attIndex`). For other prefixed names, set
     the flag to 2 and count it in `nPrefixes`.
4. **Record `m_nSpecifiedAtts` and `m_idAttIndex`.**
5. **Defaulting:** for each `DEFAULT_ATTRIBUTE` of the element type whose
   flag is still 0 and that has a value, append it, or call `addBinding` if
   it is a defaulted `xmlns`.
6. **Namespace expansion of prefixed attributes** (flag == 2): build
   `uri + sep + local [+ sep + prefix]` in `m_tempPool`, and check for
   duplicates *by expanded name* with the small open-addressing table
   `m_nsAtts`. That table is keyed by a SipHash of URI plus local name. It is
   **versioned**: instead of clearing it on every start tag, the parser
   decrements `m_nsAttsVersion` and treats entries with a different version
   as empty. It is fully reset only when the version wraps or the table
   grows.
7. **Clear all flags** (`name[-1] = 0`) for the next tag.
8. **Expand the element name** (namespace mode only), see below.

The array given to the start handler is `m_atts` reinterpreted as
`const XML_Char **`: a NULL-terminated list of name/value pairs. This works
because `sizeof(ATTRIBUTE) >= 2 * sizeof(pointer)` and the parser writes
`appAtts[k]` only at indices it has already read from `m_atts[i]`, so it never
overwrites data it still needs.

## 4.10 Namespaces

Namespace processing is on when the parser is created with
`XML_ParserCreateNS(enc, sep)` (`m_ns = XML_TRUE`, `m_namespaceSeparator =
sep`).

**Data model** (details in [06](06-data-structures.md)):

* `PREFIX { name; BINDING *binding; }` is interned in `dtd->prefixes`.
  `binding` is the binding currently in scope. `dtd->defaultPrefix` (name
  NULL) stands for the default namespace.
* `BINDING { prefix, nextTagBinding, prevPrefixBinding, attId, uri, uriLen, uriAlloc }`.
  * `prevPrefixBinding` links the bindings of one prefix into a **scope
    stack**.
  * `nextTagBinding` links all bindings declared on one element (or one
    empty-element tag), so they can be popped together.
  * `uri` holds `URI + separator` and has spare room (`EXPAND_SPARE` = 24).
    Expanded names are built **in place in this buffer**.

**`addBinding`** (`xmlparse.c:4409`) enforces the Namespaces spec rules:
`xmlns` cannot be bound, `xml` may only be bound to its fixed URI and that URI
may not be bound to anything else, `xmlns`'s URI cannot be bound, a prefix
cannot be undeclared (empty URI for a non-default prefix). It also rejects
URIs containing the separator character when the separator is not a legal
URI character, so applications can split expanded names safely. It reuses a
`BINDING` from `m_freeBindingList` when possible, pushes it onto the prefix's
scope stack and the tag's list, and calls the start-namespace handler.

**Element names:** after attribute processing, `storeAtts` finds the binding
(of the element's prefix, or the default namespace), makes sure
`binding->uri` has room, appends the local name (and `sep + prefix` if
triplets are on), and sets `tagNamePtr->str = binding->uri`. So the start
handler gets `"http://uri<sep>local"` **without a separate allocation**. If
`binding->uri` has to be reallocated, any open tag whose `name.str` pointed to
the old buffer is updated.

Because nested elements in the same namespace reuse the same `binding->uri`
buffer, an element's expanded name gets overwritten by its children. That is
why the `END_TAG` case rebuilds it (`uri + uriLen` ← `localPart`, and
`prefix` for triplets) just before calling the end handler.

**Context for child parsers.** `getContext()` serializes the current
bindings and the open general entities as
`"=defaultURI\fprefix=uri\f…\fentityName…"` (`\f` = `CONTEXT_SEP`, ASCII FF).
The result is passed to the external-entity handler as `context`.
`setContext()` in the child parses it, adds the bindings to
`m_inheritedBindings`, and marks the entities as open so that recursion
through external entities is still detected. The root parser itself starts
with `setContext(implicitContext)`, which pre-binds `xml`.
