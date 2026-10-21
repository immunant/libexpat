# 8. End-to-end walkthrough

This document follows one small document through the code, naming the
function at each step. Input, fed in **two** `XML_Parse` calls to show
incremental behavior:

```xml
<?xml version="1.0"?>
<!DOCTYPE r [
  <!ENTITY e "x<b/>y">
  <!ATTLIST r a CDATA "d">
]>
<r xmlns="urn:u">t&e;<![CDATA[c]]></r>
```

The application does:

```c
XML_Parser p = XML_ParserCreateNS(NULL, '|');
XML_SetElementHandler(p, start, end);
XML_SetCharacterDataHandler(p, chars);
XML_Parse(p, doc, 60, 0);              // chunk 1: ends in the middle of the DOCTYPE
XML_Parse(p, doc + 60, len - 60, 1);   // chunk 2: the rest, isFinal
XML_ParserFree(p);
```

## Step 0: creation

`XML_ParserCreateNS` → `XML_ParserCreate_MM(NULL, NULL, "|")` →
`parserCreate`:

* allocates the parser struct (with the size header when `XML_GE`), then
  `m_atts[16]`, `m_dataBuf[1024]`, and an empty `DTD` (`dtdCreate`);
* `parserInit`: `m_processor = prologInitProcessor`, prolog state `prolog0`,
  `XmlInitEncoding(&m_initEncoding, &m_encoding, NULL)` so `m_encoding` is
  the *detecting* encoding, status `XML_INITIALIZED`, limits set to their
  defaults;
* `m_ns = TRUE`, `m_namespaceSeparator = '|'`,
  `m_internalEncoding = XmlGetUtf8InternalEncodingNS()`.

## Step 1: first `XML_Parse` (60 bytes, not final)

1. The status is `XML_INITIALIZED`, so `startParsing`:
   * `m_hash_secret_salt = generate_hash_secret_salt()` (for example
     `getrandom`);
   * namespace mode, so `setContext("xml=http://www.w3.org/XML/1998/namespace")`
     → `addBinding(prefix "xml", attId NULL, uri)` → into
     `m_inheritedBindings`. This interns `PREFIX "xml"` in `dtd->prefixes`.
2. Status becomes `XML_PARSING`. `XML_GetBuffer(60)` allocates `m_buffer`
   (1024 bytes, from `m_mem.malloc_fcn`). The bytes are copied in, then
   `XML_ParseBuffer(60, 0)`.
3. `callProcessor(p, m_bufferPtr, m_bufferEnd, &m_bufferPtr)`. Nothing was
   deferred before, so it calls `m_processor`:
4. **`prologInitProcessor`** → `initializeEncoding()` (no protocol encoding,
   so this is a no-op) → `m_processor = prologProcessor` →
   **`prologProcessor`**:
   * `XmlPrologTok(m_encoding = initEnc …)` → `initScanProlog` → `initScan`.
     The first bytes are `3C 3F`, which match no BOM or UTF-16 pattern, so the
     chosen encoding is `encodings[NO_ENC]` = `utf8_encoding_ns`. That is
     written into `m_encoding`, and `normal_prologTok` is called. It returns
     **`XML_TOK_XML_DECL`** for `<?xml version="1.0"?>`
     (`scanPi` → `checkPiTarget` sees `xml`).
5. **`doProlog`** loop:
   * `XmlTokenRole(prolog0, XML_DECL)` → **`XML_ROLE_XML_DECL`**; next state
     `prolog1`.
   * `processXmlDecl(isGeneralTextEntity=0)` → `XmlParseXmlDeclNS` →
     `doParseXmlDecl` finds `version="1.0"`, no encoding, no standalone. No
     XML-declaration handler is set; no new encoding.
   * Next token: `PROLOG_S` (newline) → `XML_ROLE_NONE`. There is no default
     handler, so nothing is reported.
   * `DECL_OPEN` `<!DOCTYPE` → `prolog1` matches `KW_DOCTYPE` →
     `XML_ROLE_DOCTYPE_NONE`, next `doctype0`.
   * `NAME r` → `XML_ROLE_DOCTYPE_NAME`. No doctype handler, so the name is
     not stored. `m_doctypeSysid = NULL`.
   * `OPEN_BRACKET` → `XML_ROLE_DOCTYPE_INTERNAL_SUBSET`, next
     `internalSubset`.
   * `DECL_OPEN` `<!ENTITY` → `entity0`. `NAME e` →
     **`XML_ROLE_GENERAL_ENTITY_NAME`**: `poolStoreString(&dtd->pool, "e")`,
     `lookup(generalEntities, "e", sizeof(ENTITY))` creates the entry,
     `poolFinish`, `is_internal = TRUE`. Next `entity2`.
   * `LITERAL "x<b/>y"` → **`XML_ROLE_ENTITY_VALUE`** →
     `callStoreEntityValue` → `storeEntityValue`: `XmlEntityValueTok` gives
     `DATA_CHARS "x<b/>y"`, which is appended to `entityValuePool`.
     (`entityValueTok` does not treat `<` as special; markup inside the
     replacement text is tokenized later, when the entity is expanded.) Then
     `entity->textPtr`/`textLen = 6`. Next `declClose`.
   * `DECL_CLOSE` → `setTopLevel` → `internalSubset`.
   * Suppose chunk 1 ends in the middle of `<!ATTLIST r a CD`.
     `XmlPrologTok` returns **`XML_TOK_PARTIAL`** (`tok <= 0`), `haveMore` is
     true, so `*nextPtr = s` (the start of `<!ATTLIST`) and it returns
     `XML_ERROR_NONE`.
6. Back in `callProcessor`: no re-enter, so it stops. Some bytes were
   consumed, so `m_partialTokenBytesBefore = 0`.
   `XML_ParseBuffer` updates `m_position` up to `m_bufferPtr`, which now
   points at `<!ATTLIST`. It returns `XML_STATUS_OK`.

## Step 2: second `XML_Parse` (the rest, final)

1. `XML_GetBuffer(len2)`: the unparsed tail `<!ATTLIST r a CD` plus up to
   1024 bytes of context (`XML_CONTEXT_BYTES`) are moved to the front of
   `m_buffer` or into a bigger buffer. The new bytes are appended after them.
2. `XML_ParseBuffer(len2, 1)` → `finalBuffer = TRUE` (reparse deferral is
   skipped) → `prologProcessor` from `<!ATTLIST`.
3. `doProlog`:
   * `<!ATTLIST` → `attlist0`. `NAME r` → **`ATTLIST_ELEMENT_NAME`** →
     `getElementType("r")` creates `ELEMENT_TYPE r` in `elementTypes` (and,
     through `setElementTypePrefix`, finds no prefix).
   * `NAME a` → **`ATTRIBUTE_NAME`** → `getAttributeId("a")` interns `"\0a"`
     in `dtd->pool` (the leading slot is the flag) and creates the
     `ATTRIBUTE_ID`. In namespace mode it checks for `xmlns`/a prefix; there
     is none.
   * `NAME CDATA` → **`ATTRIBUTE_TYPE_CDATA`**: `m_declAttributeIsCdata = TRUE`.
   * `LITERAL "d"` → **`DEFAULT_ATTRIBUTE_VALUE`** →
     `storeAttributeValue(isCdata, "d", &dtd->pool)` →
     `defineAttribute(r, a, cdata, !id, "d")` → `r.defaultAtts[0] = {a,
     CDATA, "d"}`.
   * `DECL_CLOSE`, `PROLOG_S`, `CLOSE_BRACKET` (`]`) → `doctype5`;
     `DECL_CLOSE` (`>`) → **`XML_ROLE_DOCTYPE_CLOSE`**. There is no system ID
     and no foreign DTD, so no external subset. There is no end-doctype
     handler. Next `prolog2`.
   * `PROLOG_S`, then `<r` → `prologTok` sees `<` followed by a name-start
     character → **`XML_TOK_INSTANCE_START`**, with `next` pointing back at
     `<`. That gives **`XML_ROLE_INSTANCE_START`** → `m_processor =
     contentProcessor` and a direct call to `contentProcessor(s = "<r…")`.
4. **`doContent`** (`startTagLevel = 0`, `enc = utf8_encoding_ns`):
   * **`START_TAG_WITH_ATTS`** `<r xmlns="urn:u">`:
     * get a `TAG` (`malloc`, with a 32-byte `buf`), push it, `m_tagLevel = 1`,
       `rawName → "r"` in the input buffer, convert it into `tag->buf` →
       `"r"`.
     * **`storeAtts`**:
       * `lookup(elementTypes, "r")` finds the type declared above
         (`nDefaultAtts = 1`).
       * `XmlGetAttributes` → one attribute `xmlns="urn:u"`, already
         normalized.
       * `getAttributeId("xmlns")` → `prefix = &dtd->defaultPrefix`,
         `xmlns = TRUE`. Set flag `name[-1] = 1`. The value is stored in
         `m_tempPool`. Because it is `xmlns`, call
         **`addBinding(defaultPrefix, attId, "urn:u", &tag->bindings)`**:
         reserved-name checks pass, a new `BINDING` gets
         `uri = "urn:u|"` (`uriLen = 6`) with spare room,
         `defaultPrefix.binding = b`. There is no namespace-declaration
         handler. The attribute is removed from the list (`--attIndex`).
       * `m_nSpecifiedAtts = 0`, `m_idAttIndex = -1`.
       * Defaulting: `a` has flag 0 and value `"d"` → `appAtts = ["a", "d"]`
         with flag 1. (Unprefixed attributes are **not** in the default
         namespace, so no expansion.)
       * `nPrefixes == 0`, so the `m_nsAtts` table is not used. Clear the
         flags.
       * Expand the element name: there is no prefix, but
         `defaultPrefix.binding` exists, so append `"r"` after `"urn:u|"` in
         `binding->uri` and set `tag->name.str = "urn:u|r"`,
         `localPart = "r"`.
     * **`start(userData, "urn:u|r", {"a","d",NULL})`**.
     * `poolClear(&m_tempPool)`.
   * **`DATA_CHARS "t"`** → UTF-8 to UTF-8, so `MUST_CONVERT` is false →
     **`chars(userData, ptr_into_buffer, 1)`**.
   * **`ENTITY_REF "&e;"`**: not predefined; `lookup(generalEntities, "e")`
     finds it. The document has no PE references, so it must be declared,
     and it is, internally. It is not open and has text, and
     `m_defaultExpandInternalEntities` is true, so
     **`processEntity(e, FALSE, ENTITY_INTERNAL)`**: take an
     `OPEN_INTERNAL_ENTITY`, set `e.open = hasMore = TRUE`, push it,
     `startTagLevel = 1`, `m_processor = internalEntityProcessor`, and
     **`triggerReenter()`**.
   * At the bottom of the loop the status is `PARSING` with `m_reenter`, so
     `*nextPtr = next` (just after `&e;`) and return.
5. `contentProcessor` → `storeRawNames()` copies `"r"` into `tag->buf` after
   the converted name. Return to **`callProcessor`**: `m_reenter` is set, so
   clear it and loop: `m_processor(p, *endPtr …)` →
   **`internalEntityProcessor`**:
   * top = `e`, `hasMore`: run `doContent(startTagLevel = 1,
     enc = m_internalEncoding, "x<b/>y", haveMore = FALSE,
     XML_ACCOUNT_ENTITY_EXPANSION)`:
     * `DATA_CHARS "x"` → `chars("x", 1)`. (Every token is passed through
       `accountingDiffTolerated` as indirect bytes. The amplification here is
       far below the limits.)
     * `EMPTY_ELEMENT_NO_ATTS <b/>` → name in `m_tempPool`, `storeAtts`
       (element type `b` is created on the fly; the default namespace
       applies, giving `"urn:u|b"`) → **`start("urn:u|b", {NULL})`**,
       **`end("urn:u|b")`**.
     * `DATA_CHARS "y"` → `chars("y", 1)`.
     * `XML_TOK_NONE` with `haveMore == FALSE`, `startTagLevel > 0`, and
       `m_tagLevel == 1 == startTagLevel`, so it is balanced and returns.
   * The entity is finished: `e.hasMore = FALSE`, `triggerReenter()`.
   * Loop again → `internalEntityProcessor`: `hasMore` is false, so pop `e`
     (`e.open = FALSE`, record back to the free list). The stack is empty, so
     `m_processor = contentProcessor`. `triggerReenter()`.
   * Loop again → **`contentProcessor`** resumes in the document buffer just
     after `&e;`.
6. `doContent` continues:
   * **`CDATA_SECT_OPEN`** → there is no CDATA handler →
     **`doCdataSection`**: `DATA_CHARS "c"` → `chars("c", 1)`;
     `CDATA_SECT_CLOSE` → return.
   * **`END_TAG </r>`**: compare the raw name bytes with
     `m_tagStack->rawName` (`"r"` == `"r"`). Pop the tag to the free list,
     `m_tagLevel = 0`. Namespace mode: rebuild `"urn:u|r"` in
     `binding->uri` (it may have been overwritten by `"urn:u|b"` meanwhile)
     → **`end("urn:u|r")`**. Unwind `tag->bindings`:
     `defaultPrefix.binding = NULL` (the previous value), `BINDING` to the
     free list.
   * `m_tagLevel == 0` → **`epilogProcessor`**: `PROLOG_S` (final newline,
     if any), then `XML_TOK_NONE` → done.
7. `XML_ParseBuffer`: no error and `isFinal`, so status becomes
   **`XML_FINISHED`** and it returns `XML_STATUS_OK`.

Handler calls seen by the application, in order:

```
start("urn:u|r", ["a","d"])
chars("t")
chars("x")
start("urn:u|b", [])
end("urn:u|b")
chars("y")
chars("c")
end("urn:u|r")
```

Note that character data comes in **several pieces**: one call per token.
Applications must concatenate adjacent calls. A reimplementation does not
have to split text at exactly the same places, but applications, and Expat's
own tests, often depend on the exact pieces. Matching Expat means one call per
`DATA_CHARS`/`DATA_NEWLINE` token plus extra splits when `m_dataBuf` fills
during conversion.

## Step 3: `XML_ParserFree`

Walks `m_tagStack` + `m_freeTagList` (freeing each `buf`, its bindings, and
the `TAG`), the three open/free entity lists, `m_freeBindingList`,
`m_inheritedBindings` (the implicit `xml` binding), the temp pools, and the
protocol encoding name. Then `dtdDestroy` (it owns the DTD: not a PE parser,
and the root parser, so it also frees the scaffold), `m_atts`,
`m_groupConnector`, `m_buffer` (raw `free_fcn`), `m_dataBuf`, `m_nsAtts`, the
unknown-encoding memory (calling the user's `release`), and finally the
parser struct.

## Variations worth tracing yourself

| Scenario | Where to look |
|----------|---------------|
| UTF-16 input with a BOM | `initScan` → `XML_TOK_BOM`; `little2_*` scanners; `MUST_CONVERT` true, so text goes through `m_dataBuf` |
| `XML_StopParser(p, XML_TRUE)` inside `start` | the status switch at the end of the `doContent` loop; `XML_ResumeParser` → `callProcessor` from `m_bufferPtr` |
| `<!DOCTYPE r SYSTEM "r.dtd">` with `XML_SetParamEntityParsing(ALWAYS)` | `XML_ROLE_DOCTYPE_CLOSE` → external entity handler with `context == NULL` → child `externalParEntInitProcessor` sharing the DTD |
| `&ext;` declared `SYSTEM "x.xml"` | `doContent` `ENTITY_REF` → `getContext()` → handler → child `externalEntityInitProcessor*` with a `dtdCopy` |
| Billion laughs | `accountingDiffTolerated` → `XML_ERROR_AMPLIFICATION_LIMIT_BREACH` |
| A tag name split across 1-byte `XML_Parse` calls | `XML_TOK_PARTIAL` → `callProcessor` deferral heuristic |
