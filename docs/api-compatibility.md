# Expat API compatibility

[Guide index](README.md)

[`expat.h`](../expat/lib/expat.h) defines the public contracts, [`expat_external.h`](../expat/lib/expat_external.h) defines their platform-dependent types, and [`xmlparse.c`](../expat/lib/xmlparse.c) supplies the behavior. A source-compatible wrapper and a binary-compatible replacement have different obligations; both must preserve callback ownership and incremental parsing semantics.

## Public API groups

| Group | Entry points and behavior to preserve |
| --- | --- |
| Construction | `XML_ParserCreate`, `XML_ParserCreateNS`, `XML_ParserCreate_MM`; encoding, namespace separator, custom allocator, null on failure |
| Reuse and destruction | `XML_ParserReset`, `XML_ParserFree`; root-only reset, handler exceptions, retained allocations, partial-failure cleanup |
| Input | `XML_Parse`, `XML_GetBuffer`, `XML_ParseBuffer`; byte lengths, final flag, pending input, reservation ownership |
| Execution control | `XML_StopParser`, `XML_ResumeParser`, `XML_GetParsingStatus`; suspension versus abort and parser-family coordination |
| Element and text events | Element/start/end handler setters, character-data, PI, comment, CDATA boundary setters |
| Default reporting | `XML_SetDefaultHandler`, `XML_SetDefaultHandlerExpand`, `XML_DefaultCurrent`; original spelling and internal-expansion policy |
| DTD events | DOCTYPE/start/end, element-declaration, attribute-list, entity, unparsed-entity, notation, XML-declaration, not-standalone, skipped-entity setters |
| Namespace events | Namespace/start/end setters and `XML_SetReturnNSTriplet` |
| Application context | `XML_SetUserData`, `XML_GetUserData`, `XML_UseParserAsHandlerArg`, `XML_SetExternalEntityRefHandlerArg` |
| External resources | `XML_SetExternalEntityRefHandler`, `XML_ExternalEntityParserCreate`, `XML_SetParamEntityParsing`, `XML_UseForeignDTD`, `XML_SetBase`, `XML_GetBase` |
| Encoding control | `XML_SetEncoding`, `XML_SetUnknownEncodingHandler` |
| Event metadata | Specified-attribute count, ID-attribute index, optional attribute byte ranges, current line/column/byte index/count, input context |
| Errors and features | `XML_GetErrorCode`, `XML_ErrorString`, version functions, `XML_GetFeatureList` |
| Application memory | `XML_FreeContentModel`, `XML_MemMalloc`, `XML_MemRealloc`, `XML_MemFree` |
| Resource policy | `XML_SetHashSalt`, reparse deferral, expansion-amplification and allocation-tracker setters |

Paired setters install both handlers, while single setters update just one. Most handler setters tolerate a null parser and return no status; other functions have function-specific null/error returns. A replacement should derive these individually from the source instead of applying one universal null policy.

## Callback payloads and arguments

Element names, attribute names/values, comments, PI strings, and most identifiers are null-terminated `XML_Char` strings. Character-data and default-handler payloads have explicit lengths and are not guaranteed to be terminated. Entity declaration values also have an explicit length. All these lengths count `XML_Char` units, not Unicode scalar values and not necessarily input bytes.

The start-element attribute array is alternating name/value pointers ending with a null pointer. Namespace declarations are omitted only in namespace mode. Explicit ordinary attributes precede defaults. No character-data callback boundary promises a complete text node, entity expansion, or source input chunk.

Ordinary callbacks receive `m_handlerArg`, initially user data. `XML_UseParserAsHandlerArg` changes it to the parser while `XML_GetUserData` still returns application data. The external-reference callback receives a parser by default and has a separate argument override. The unknown-encoding callback receives the data supplied to its setter. These argument channels must stay distinct.

Most callbacks return void. External-reference, unknown-encoding, and not-standalone callbacks return success/failure values with different consequences. Element-declaration callbacks receive an owned content model that the application must free. Other strings should be treated as borrowed for the callback duration.

## Default handler behavior

The default handler receives input for which no applicable specialized handler handles the construct, converted to API encoding but preserving spelling and line endings. It does not receive a BOM. It is not a second event stream containing every source byte regardless of registered handlers.

`XML_SetDefaultHandler` also disables normal expansion of internal general entities in content, making skipped/default reporting possible. `XML_SetDefaultHandlerExpand` installs the same kind of fallback handler while retaining internal expansion. Attribute normalization still expands references as required for the attribute value. Setting a handler therefore can change semantics, not just observation.

`XML_DefaultCurrent` explicitly reports the current event through the default handler, using internal entity event pointers during expansion. Its documented use is from the applicable element, character-data, or PI callback. Preserve event ranges so this works even when the specialized callback received converted/normalized text.

## Lifecycle and configuration timing

| Operation | Important restriction or result |
| --- | --- |
| Parse while suspended | `XML_ERROR_SUSPENDED`; resume existing work first |
| Parse after finished | `XML_ERROR_FINISHED` |
| Stop before starting | `XML_ERROR_NOT_STARTED` |
| Resume outside suspended state | `XML_ERROR_NOT_SUSPENDED` |
| Resumable stop in external parameter parser | `XML_ERROR_SUSPEND_PE` |
| `XML_ParseBuffer` before a successful initial buffer reservation | `XML_ERROR_NO_BUFFER` |
| Negative `XML_Parse`/`XML_ParseBuffer` length | `XML_ERROR_INVALID_ARGUMENT` |
| Negative `XML_GetBuffer` length | `XML_ERROR_NO_MEMORY` in this implementation |
| Nonzero parse length with null input | `XML_ERROR_INVALID_ARGUMENT` |

Configure encoding, namespace triplets, parameter-entity policy, foreign DTD policy, and hash salt before parsing. Their setters have different return types and feature-disabled behavior. Several use `parserBusy`, which tests `XML_PARSING` or `XML_SUSPENDED`, so exact rejection behavior after a finished parse is not identical to a permanent “ever started” bit. A finished parser still needs reset before reuse.

Handlers and user data can change during parsing. Reparse deferral can also change on the fly. Amplification and allocation policy setters operate on a root parser, reject null/child parsers, and reject NaN or factors below 1; the implementation does not impose the same pre-parse restriction as the encoding setters. Do not accidentally add a global “all settings frozen” rule.

`XML_ParserReset` rejects child parsers. It clears ordinary handlers, user data, base and DTD contents, error/position state, and parameter-entity policy; it preserves namespace mode and triplet selection, the memory suite, and unknown-encoding handler configuration. It releases the active unknown encoding. Reparse deferral and entity-expansion policy return to initialization defaults. Allocation-tracker configuration lives outside `parserInit` and survives reset along with its retained-allocation accounting. Reinstall application settings deliberately after reset.

## Positions and byte counts

Positions refer to original encoded input, even when callbacks receive a different encoding. Line numbers start at 1, columns at 0, and byte indexes at 0. `XML_Index` is signed, allowing `-1` when a byte position is unavailable. Position queries on null parsers have their documented sentinel returns.

`m_parseEndByteIndex` records the absolute byte position after accepted input. A current event's index is computed as:

```text
parseEndByteIndex - (parseEndPtr - eventPtr)
```

The parser advances line/column state lazily from `m_positionPtr` to the event pointer through the active encoding's `updatePosition`. This counts encoded characters according to the tokenizer's position routine and treats CRLF as one line boundary. It is not a count of callback bytes.

During callbacks, the range generally corresponds to the generating source token; declaration callback positions can be inside the declaration. After an error it identifies the detected failure location. Outside a callback, the position follows the public API's last-event semantics rather than simply pointing to the end of the most recently supplied chunk.

Internal entity callbacks retain the containing document's reference location for public position queries. Separate internal event pointers support default reporting of the replacement text. External child parsers have their own byte/line origin.

There is a contract/implementation difference in this snapshot: the header says `XML_GetCurrentByteCount` returns zero for internal-entity events, but the getter simply subtracts the public event pointers. In the baseline build, the character callback for `hello` in the [execution walkthrough](parser-execution.md#a-complete-document-walkthrough) reports **7**, the byte length of `&greet;`. This was checked with a callback probe against the documented commit. Treat byte counts during expansion as a differential-test case rather than assuming the header's zero rule describes every path in this revision.

For an empty element with both element handlers installed, the end-element event is moved to the end of the start-tag token, with zero length. A replacement that reports the same full source range for both callbacks changes observable metadata.

`XML_GetInputContext` depends on `XML_CONTEXT_BYTES` and returns borrowed parser-buffer bytes, with an offset and total available input size. Its pointer must not be retained beyond the requesting handler. `XML_GetAttributeInfo`, when built in, is documented to return original byte ranges for explicitly specified attributes, with one record per pair rather than two. Values exclude their surrounding quotes, and defaulted attributes have no corresponding lexical input range. Include namespace-declaration removal and internal-entity-origin attributes in compatibility tests: `storeAtts` populates `m_attInfo` using the lexical attribute index and document pointer arithmetic, whereas callback attribute names/values are subsequently filtered and expanded. Do not infer these edge cases solely from the public array-length description.

## Error families

| Family | Representative codes |
| --- | --- |
| Lexical and final-input errors | `INVALID_TOKEN`, `UNCLOSED_TOKEN`, `PARTIAL_CHAR`, `BAD_CHAR_REF`, `UNCLOSED_CDATA_SECTION` |
| Document structure | `NO_ELEMENTS`, `TAG_MISMATCH`, `JUNK_AFTER_DOC_ELEMENT`, `MISPLACED_XML_PI` |
| Declarations | `SYNTAX`, `XML_DECL`, `TEXT_DECL`, `PUBLICID`, `PARAM_ENTITY_REF`, `INCOMPLETE_PE` |
| Entities | `UNDEFINED_ENTITY`, `RECURSIVE_ENTITY_REF`, `ASYNC_ENTITY`, `BINARY_ENTITY_REF`, `ATTRIBUTE_EXTERNAL_ENTITY_REF`, `ENTITY_DECLARED_IN_PE` |
| Encoding and namespaces | `UNKNOWN_ENCODING`, `INCORRECT_ENCODING`, `UNBOUND_PREFIX`, reserved-binding errors, `DUPLICATE_ATTRIBUTE` |
| Application/resource interaction | `EXTERNAL_ENTITY_HANDLING`, `NOT_STANDALONE`, `NO_MEMORY`, `AMPLIFICATION_LIMIT_BREACH`, lifecycle/argument errors |

The table abbreviates the common `XML_ERROR_` prefix. Preserve the full enum's numeric ordering for ABI compatibility. Error location and precedence matter: malformed input can violate several rules, but Expat reports the one reached by its tokenization and semantic order. The error processor prevents continued normal parsing after a parse failure; parser reset is the supported route to reuse.

## Binary interface

- `XML_Parser` is an opaque pointer, but the `XML_GetUserData` macro imposes a real first-field layout constraint.
- `XML_Bool` is an unsigned char; public flags that historically use `int`, such as the entity declaration callback's parameter flag, retain that type.
- `XML_Char` is `char` for UTF-8 or unsigned short/two-byte `wchar_t` for UTF-16. `XML_LChar` is `char` except in the wchar variant; error/version string types therefore differ from ordinary callbacks in the ushort variant.
- `XML_Index`/`XML_Size` are signed/unsigned long or long long under `XML_LARGE_SIZE`; do not hardcode the platform width of long.
- Preserve `XML_Content`, `XML_Encoding`, `XML_Memory_Handling_Suite`, `XML_ParsingStatus`, `XML_Expat_Version`, `XML_Feature`, and optional `XML_AttrInfo` layouts when exposing them through C.
- Respect `XMLCALL`, symbol visibility, static/dynamic linking definitions, Windows export ordinals in [`libexpat.def.cmake`](../expat/lib/libexpat.def.cmake), and ELF symbol versions in [`libexpat.map.in`](../expat/lib/libexpat.map.in).

Version strings and the terminated feature list describe the implementation/build. A new implementation should report its compatibility deliberately; passing basic XML tests does not establish that it can claim every Expat feature or replace every binary variant.
