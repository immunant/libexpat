# 10. Public API reference (implementation-oriented)

A compact list of every exported function in `expat.h` (Expat 2.7.4), with
the argument checks, return values, and side effects the implementation
actually has. The export list is in `expat/lib/libexpat.map.in` /
`libexpat.def.cmake`. All functions use the `XMLCALL` calling convention.
Most accept `parser == NULL` and return a neutral value.

## 10.1 Types and constants

| Type | Definition |
|------|------------|
| `XML_Parser` | `struct XML_ParserStruct *` (opaque, but **the first field must be `void *userData`** because of the `XML_GetUserData(p)` macro: `(*(void **)(p))`) |
| `XML_Bool` | `unsigned char`, `XML_TRUE` = 1, `XML_FALSE` = 0 |
| `XML_Char` / `XML_LChar` | `char`/`char`; `unsigned short`/`char` (`XML_UNICODE`); `wchar_t`/`wchar_t` (`XML_UNICODE_WCHAR_T`) |
| `XML_Index` / `XML_Size` | `long`/`unsigned long`, or `long long`/`unsigned long long` with `XML_LARGE_SIZE` |
| `enum XML_Status` | `ERROR = 0`, `OK = 1`, `SUSPENDED = 2` |
| `enum XML_Error` | 0 … 44, **order is ABI**: `NONE, NO_MEMORY, SYNTAX, NO_ELEMENTS, INVALID_TOKEN, UNCLOSED_TOKEN, PARTIAL_CHAR, TAG_MISMATCH, DUPLICATE_ATTRIBUTE, JUNK_AFTER_DOC_ELEMENT, PARAM_ENTITY_REF, UNDEFINED_ENTITY, RECURSIVE_ENTITY_REF, ASYNC_ENTITY, BAD_CHAR_REF, BINARY_ENTITY_REF, ATTRIBUTE_EXTERNAL_ENTITY_REF, MISPLACED_XML_PI, UNKNOWN_ENCODING, INCORRECT_ENCODING, UNCLOSED_CDATA_SECTION, EXTERNAL_ENTITY_HANDLING, NOT_STANDALONE, UNEXPECTED_STATE, ENTITY_DECLARED_IN_PE, FEATURE_REQUIRES_XML_DTD, CANT_CHANGE_FEATURE_ONCE_PARSING, UNBOUND_PREFIX, UNDECLARING_PREFIX, INCOMPLETE_PE, XML_DECL, TEXT_DECL, PUBLICID, SUSPENDED, NOT_SUSPENDED, ABORTED, FINISHED, SUSPEND_PE, RESERVED_PREFIX_XML, RESERVED_PREFIX_XMLNS, RESERVED_NAMESPACE_URI, INVALID_ARGUMENT, NO_BUFFER, AMPLIFICATION_LIMIT_BREACH, NOT_STARTED` |
| `enum XML_Content_Type` | `EMPTY = 1, ANY, MIXED, NAME, CHOICE, SEQ` |
| `enum XML_Content_Quant` | `NONE, OPT, REP, PLUS` |
| `XML_Content` (`struct XML_cp`) | `{ type; quant; XML_Char *name; unsigned numchildren; XML_Content *children; }` |
| `XML_Memory_Handling_Suite` | `{ malloc_fcn; realloc_fcn; free_fcn; }` |
| `XML_Encoding` | `{ int map[256]; void *data; int (*convert)(void*, const char*); void (*release)(void*); }` |
| `enum XML_Parsing` | `INITIALIZED, PARSING, FINISHED, SUSPENDED` |
| `XML_ParsingStatus` | `{ enum XML_Parsing parsing; XML_Bool finalBuffer; }` |
| `enum XML_ParamEntityParsing` | `NEVER, UNLESS_STANDALONE, ALWAYS` |
| `XML_Expat_Version` | `{ int major, minor, micro; }` |
| `enum XML_FeatureEnum` / `XML_Feature` | `{ feature; const XML_LChar *name; long value; }`, list terminated by `XML_FEATURE_END` |
| `XML_AttrInfo` (`XML_ATTR_INFO`) | `{ XML_Index nameStart, nameEnd, valueStart, valueEnd; }` (byte offsets, value excludes quotes) |
| Version macros | `XML_MAJOR_VERSION 2`, `XML_MINOR_VERSION 7`, `XML_MICRO_VERSION 4` |
| Compat macros | `XML_GetErrorLineNumber`, `XML_GetErrorColumnNumber`, `XML_GetErrorByteIndex` = the `GetCurrent*` functions |

## 10.2 Creation and destruction

| Function | Behavior |
|----------|----------|
| `XML_Parser XML_ParserCreate(const XML_Char *encoding)` | `= XML_ParserCreate_MM(encoding, NULL, NULL)` |
| `XML_Parser XML_ParserCreateNS(const XML_Char *encoding, XML_Char sep)` | `= XML_ParserCreate_MM(encoding, NULL, &sep)`; enables namespace processing |
| `XML_Parser XML_ParserCreate_MM(enc, const XML_Memory_Handling_Suite *ms, const XML_Char *sep)` | `ms == NULL` uses malloc/realloc/free. `sep != NULL` enables namespaces with `*sep` (may be `'\0'`). Copies `enc`. Returns NULL on OOM. |
| `XML_Parser XML_ExternalEntityParserCreate(XML_Parser parent, const XML_Char *context, const XML_Char *encoding)` | Child parser for an external entity (see [05 §5.5](05-dtd-and-entities.md#55-external-entities-and-child-parsers)). `context == NULL` means parameter entity / external subset (shares the DTD; `XML_DTD` only). Otherwise a general entity (copies the DTD, applies `context`). Inherits handlers, user data, memory suite, NS settings, salt, and reparse deferral. NULL on OOM or bad context. |
| `XML_Bool XML_ParserReset(XML_Parser p, const XML_Char *encoding)` | Root parsers only (returns `XML_FALSE` for child parsers). Clears all handlers **except** the unknown-encoding handler. Keeps `ns`, the separator, and `ns_triplets`. Resets the DTD, limits, and salt (a new salt is generated on the next parse). Keeps allocations for reuse. |
| `void XML_ParserFree(XML_Parser p)` | Frees everything. Does not free a DTD shared with a parent. Calls the unknown encoding's `release`. NULL-safe. |

## 10.3 Feeding input

| Function | Behavior |
|----------|----------|
| `enum XML_Status XML_Parse(p, const char *s, int len, int isFinal)` | `len < 0` or `(s == NULL && len != 0)` → ERROR, `XML_ERROR_INVALID_ARGUMENT`. Status checks as in [09 §9.2](09-behavioral-spec.md#92-lifecycle-and-state-machine). Copies into the internal buffer (or parses in place if `XML_CONTEXT_BYTES == 0` and the buffer is empty), then parses. `len` may be 0 (useful for the final call). |
| `void *XML_GetBuffer(p, int len)` | Returns space for at least `len` bytes after any unparsed data. `len < 0` → NULL, `NO_MEMORY`. Suspended/finished → NULL with that error. May move the buffer, which invalidates earlier pointers. Records `len` for the reparse-deferral heuristic. |
| `enum XML_Status XML_ParseBuffer(p, int len, int isFinal)` | Parses `len` bytes that the application wrote into the last `XML_GetBuffer` result. `len < 0` → `INVALID_ARGUMENT`. Never having called `XML_GetBuffer` (in INITIALIZED) → `XML_ERROR_NO_BUFFER`. |
| `enum XML_Status XML_StopParser(p, XML_Bool resumable)` | See [09 §9.2](09-behavioral-spec.md#92-lifecycle-and-state-machine). Meant to be called from handlers, or to abort a suspended parser. |
| `enum XML_Status XML_ResumeParser(p)` | Not suspended → `NOT_SUSPENDED`. Continues parsing the remaining buffered data; if it was the final buffer and parsing completes, status becomes FINISHED. Must not be called from a handler. For nested parsers, resume the innermost child first. |
| `void XML_GetParsingStatus(p, XML_ParsingStatus *status)` | Copies the status. `status` must be non-NULL. |

## 10.4 Handler registration

Each setter stores the pointer. They can be called at any time, including
from inside handlers, and take effect at the next event. Paired setters set
both. NULL removes a handler.

```
XML_SetElementHandler(p, start, end)          XML_SetStartElementHandler / XML_SetEndElementHandler
XML_SetCharacterDataHandler(p, h)             XML_SetProcessingInstructionHandler(p, h)
XML_SetCommentHandler(p, h)
XML_SetCdataSectionHandler(p, start, end)     XML_SetStartCdataSectionHandler / XML_SetEndCdataSectionHandler
XML_SetDefaultHandler(p, h)        // also disables internal entity expansion in content
XML_SetDefaultHandlerExpand(p, h)  // keeps expansion on
XML_SetDoctypeDeclHandler(p, start, end)      XML_SetStartDoctypeDeclHandler / XML_SetEndDoctypeDeclHandler
XML_SetUnparsedEntityDeclHandler(p, h)        XML_SetNotationDeclHandler(p, h)
XML_SetNamespaceDeclHandler(p, start, end)    XML_SetStartNamespaceDeclHandler / XML_SetEndNamespaceDeclHandler
XML_SetNotStandaloneHandler(p, h)
XML_SetExternalEntityRefHandler(p, h)
XML_SetExternalEntityRefHandlerArg(p, void *arg)   // NULL → the parser itself is passed
XML_SetSkippedEntityHandler(p, h)
XML_SetUnknownEncodingHandler(p, h, void *encodingHandlerData)
XML_SetElementDeclHandler(p, h)    XML_SetAttlistDeclHandler(p, h)
XML_SetEntityDeclHandler(p, h)     XML_SetXmlDeclHandler(p, h)
```

Handler signatures are in `expat.h` and their semantics in
[09 §9.3](09-behavioral-spec.md#93-events). Every handler except
`ExternalEntityRefHandler` (which gets the parser or the arg) and
`UnknownEncodingHandler` (which gets `encodingHandlerData`) receives
`handlerArg` as its first argument.

## 10.5 Configuration

| Function | Behavior |
|----------|----------|
| `void XML_SetUserData(p, void *ud)` | Sets `userData`. If `handlerArg` was equal to the old `userData`, it follows the change. |
| `void XML_UseParserAsHandlerArg(p)` | `handlerArg = p` from now on |
| `enum XML_Status XML_SetEncoding(p, const XML_Char *enc)` | Protocol encoding. Refused (ERROR) while parsing or suspended. Copies the string. |
| `enum XML_Status XML_SetBase(p, const XML_Char *base)` / `const XML_Char *XML_GetBase(p)` | Base URI, interned into the DTD pool and passed back to handlers. ERROR on OOM. |
| `void XML_SetReturnNSTriplet(p, int do_nst)` | Ignored once parsing has started |
| `enum XML_Error XML_UseForeignDTD(p, XML_Bool)` | `NONE` / `CANT_CHANGE_FEATURE_ONCE_PARSING` / `FEATURE_REQUIRES_XML_DTD` / `INVALID_ARGUMENT` |
| `int XML_SetParamEntityParsing(p, enum)` | 1 on success; 0 if parsing has started, `p` is NULL, or a non-NEVER mode is asked for without `XML_DTD` |
| `int XML_SetHashSalt(p, unsigned long salt)` | Sets the **root** parser's salt; 0 if parsing has started or `p` is NULL |
| `XML_Bool XML_SetBillionLaughsAttackProtectionMaximumAmplification(p, float f)` | Root parser only; `f` must be ≥ 1.0 and not NaN (`XML_GE`) |
| `XML_Bool XML_SetBillionLaughsAttackProtectionActivationThreshold(p, unsigned long long n)` | Root parser only (`XML_GE`) |
| `XML_Bool XML_SetAllocTrackerMaximumAmplification(p, float f)` | Root parser only; `f` ≥ 1.0, not NaN (`XML_GE`, 2.7.2+) |
| `XML_Bool XML_SetAllocTrackerActivationThreshold(p, unsigned long long n)` | Root parser only (`XML_GE`, 2.7.2+) |
| `XML_Bool XML_SetReparseDeferralEnabled(p, XML_Bool enabled)` | Any time; `enabled` must be exactly 0 or 1 |

## 10.6 Queries

| Function | Behavior |
|----------|----------|
| `enum XML_Error XML_GetErrorCode(p)` | Last error (`INVALID_ARGUMENT` if `p` is NULL) |
| `const XML_LChar *XML_ErrorString(enum XML_Error)` | Fixed English strings (`xmlparse.c:2872`); NULL for `NONE` or an unknown code |
| `XML_Size XML_GetCurrentLineNumber(p)` | 1-based; updates the position lazily from `positionPtr` to `eventPtr` |
| `XML_Size XML_GetCurrentColumnNumber(p)` | 0-based, counted in characters |
| `XML_Index XML_GetCurrentByteIndex(p)` | `parseEndByteIndex - (parseEndPtr - eventPtr)`, or -1 if there is no event |
| `int XML_GetCurrentByteCount(p)` | `eventEndPtr - eventPtr`, or 0 |
| `const char *XML_GetInputContext(p, int *offset, int *size)` | See [09 §9.7](09-behavioral-spec.md#97-position-and-error-reporting) |
| `int XML_GetSpecifiedAttributeCount(p)` | `2 × #specified` attributes of the last start tag; -1 if `p` is NULL |
| `int XML_GetIdAttributeIndex(p)` | Index of the specified ID attribute in `atts`, or -1 |
| `const XML_AttrInfo *XML_GetAttributeInfo(p)` | `XML_ATTR_INFO` only; one entry per *specified* attribute |
| `void XML_DefaultCurrent(p)` | Sends the current event's raw text to the default handler (if set) |
| `void XML_FreeContentModel(p, XML_Content *m)` | `m_mem.free_fcn(m)` (one block) |
| `void *XML_MemMalloc(p, size_t)` / `XML_MemRealloc(p, ptr, size_t)` / `void XML_MemFree(p, ptr)` | Direct access to the parser's memory suite (**not** tracked) |
| `const XML_LChar *XML_ExpatVersion(void)` | `"expat_2.7.4"` |
| `XML_Expat_Version XML_ExpatVersionInfo(void)` | `{2, 7, 4}` |
| `const XML_Feature *XML_GetFeatureList(void)` | Static array: `sizeof(XML_Char)`, `sizeof(XML_LChar)`, then flags present in the build (`XML_UNICODE`, `XML_UNICODE_WCHAR_T`, `XML_DTD`, `XML_CONTEXT_BYTES`=n, `XML_MIN_SIZE`, `XML_NS`, `XML_LARGE_SIZE`, `XML_ATTR_INFO`, `XML_BLAP_MAX_AMP`=100, `XML_BLAP_ACT_THRES`=8388608, `XML_GE`, `XML_AT_MAX_AMP`=100, `XML_AT_ACT_THRES`=67108864), then `{XML_FEATURE_END, NULL, 0}` |

## 10.7 Non-public symbols used by the test suite

Built only into `libtestpat` with `-DXML_TESTING`, or present
unconditionally but not exported:

* `testingAccountingGetCountBytesDirect(p)` / `…Indirect(p)` (`XML_GE`)
* `unsignedCharToPrintable(c)`
* `_INTERNAL_trim_to_complete_utf8_characters(from, &fromLim)`
* `g_reparseDeferralEnabledDefault` (writable under `XML_TESTING`),
  `g_bytesScanned`
* `expat_malloc` / `expat_free` / `expat_realloc` (`XML_TESTING`)
