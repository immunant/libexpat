# Public API and build compatibility

The authoritative signatures, numeric enums, and callback contracts are in
[`expat.h`](../expat/lib/expat.h). ABI macros and character/index types are in
[`expat_external.h`](../expat/lib/expat_external.h). Implementations are in
[`xmlparse.c`](../expat/lib/xmlparse.c); the longer user reference is
[`reference.html`](../expat/doc/reference.html).

## API inventory

This table groups the public surface by implementation responsibility. Exact
parameter types and enum ordinals should be taken from the header, not
reconstructed from this summary.

| Family | Entry points |
| --- | --- |
| Construction/lifetime | `XML_ParserCreate`, `XML_ParserCreateNS`, `XML_ParserCreate_MM`, `XML_ExternalEntityParserCreate`, `XML_ParserReset`, `XML_ParserFree` |
| Input/control | `XML_Parse`, `XML_GetBuffer`, `XML_ParseBuffer`, `XML_StopParser`, `XML_ResumeParser`, `XML_GetParsingStatus` |
| Element events | `XML_SetElementHandler`, `XML_SetStartElementHandler`, `XML_SetEndElementHandler` |
| Text/miscellaneous events | `XML_SetCharacterDataHandler`, `XML_SetProcessingInstructionHandler`, `XML_SetCommentHandler` |
| CDATA events | `XML_SetCdataSectionHandler`, `XML_SetStartCdataSectionHandler`, `XML_SetEndCdataSectionHandler` |
| Default events | `XML_SetDefaultHandler`, `XML_SetDefaultHandlerExpand`, `XML_DefaultCurrent` |
| DOCTYPE events | `XML_SetDoctypeDeclHandler`, `XML_SetStartDoctypeDeclHandler`, `XML_SetEndDoctypeDeclHandler` |
| DTD declaration events | `XML_SetElementDeclHandler`, `XML_SetAttlistDeclHandler`, `XML_SetEntityDeclHandler`, `XML_SetUnparsedEntityDeclHandler`, `XML_SetNotationDeclHandler`, `XML_SetXmlDeclHandler` |
| Namespace events | `XML_SetNamespaceDeclHandler`, `XML_SetStartNamespaceDeclHandler`, `XML_SetEndNamespaceDeclHandler` |
| Entity/standalone policy | `XML_SetExternalEntityRefHandler`, `XML_SetExternalEntityRefHandlerArg`, `XML_SetSkippedEntityHandler`, `XML_SetNotStandaloneHandler`, `XML_SetParamEntityParsing`, `XML_UseForeignDTD` |
| Application callback state | `XML_SetUserData`, `XML_GetUserData` (macro), `XML_UseParserAsHandlerArg` |
| Encoding/namespace settings | `XML_SetEncoding`, `XML_SetUnknownEncodingHandler`, `XML_SetReturnNSTriplet` |
| Base/attribute metadata | `XML_SetBase`, `XML_GetBase`, `XML_GetSpecifiedAttributeCount`, `XML_GetIdAttributeIndex`, optional `XML_GetAttributeInfo` |
| Error/location | `XML_GetErrorCode`, `XML_ErrorString`, `XML_GetCurrentLineNumber`, `XML_GetCurrentColumnNumber`, `XML_GetCurrentByteIndex`, `XML_GetCurrentByteCount`, `XML_GetInputContext` |
| Memory | `XML_FreeContentModel`, `XML_MemMalloc`, `XML_MemRealloc`, `XML_MemFree` |
| Protection | `XML_SetHashSalt`, `XML_SetReparseDeferralEnabled`, `XML_SetBillionLaughsAttackProtectionMaximumAmplification`, `XML_SetBillionLaughsAttackProtectionActivationThreshold`, `XML_SetAllocTrackerMaximumAmplification`, `XML_SetAllocTrackerActivationThreshold` |
| Version/build information | `XML_ExpatVersion`, `XML_ExpatVersionInfo`, `XML_GetFeatureList` |

`XML_STATUS_ERROR`, `XML_STATUS_OK`, and `XML_STATUS_SUSPENDED` are 0, 1, and 2.
Error codes form a separate enum and must not be confused with status values.
Several setters use `XML_Bool`, `int`, or `enum XML_Error` rather than
`enum XML_Status`; return-type consistency cannot be assumed.

## Callback contracts

| Event | Payload and ownership |
| --- | --- |
| Start element | NUL-terminated name and alternating NUL-terminated name/value pointers, array terminated by null |
| End element | NUL-terminated name; includes an end event for empty-element syntax |
| Character/default data | Borrowed pointer plus length in `XML_Char` units; no NUL guarantee or fixed partitioning |
| PI | NUL-terminated target and data; source line endings normalized for the specialized callback |
| Comment | NUL-terminated comment payload, with normalized line endings |
| CDATA start/end | Boundary events; data still goes through the character handler |
| XML/text declaration | Optional version/encoding strings; standalone -1/0/1 |
| DOCTYPE start/end | Name, optional external IDs, internal-subset indicator; end follows external subset handling |
| ATTLIST declaration | One call per attribute; textual type, optional normalized default, and a flag distinguishing required/fixed forms |
| Entity declaration | Internal replacement pointer plus length, or external IDs; an empty internal value is nonnull with length zero |
| Element declaration | `XML_Content` tree ownership transfers to application |
| Namespace declaration | Null prefix means default namespace; null URI means default undeclaration |
| External entity reference | Parser/override argument, context, base, IDs; application returns success/failure and supplies resource data if desired |
| Unknown encoding | Separate handler-data argument and writable `XML_Encoding` descriptor; accepted decoder has release lifecycle |
| Not standalone | Application returns nonzero to continue, zero to reject |
| Skipped entity | Name and parameter/general indicator when reporting is possible in the grammar context |

Default reporting receives otherwise unhandled source syntax converted to the
API encoding, **without line-ending normalization**. It is not simply the same
text as a character handler. BOMs are not reported. Installing
`XML_SetDefaultHandler` also disables ordinary internal entity expansion in
content; installing `XML_SetDefaultHandlerExpand` enables it. Even passing a null
handler changes this expansion flag in these setters.

`XML_DefaultCurrent` lets selected specialized callbacks explicitly forward their
current markup to the default handler. It chooses internal versus document
event pointers appropriately. Declaration callbacks also suppress default
reporting at particular grammar roles; a replacement cannot just run a default
callback for every token alongside specialized handlers.

Most callbacks receive `m_handlerArg`, initially the user data. Calling
`XML_UseParserAsHandlerArg` changes that to the parser while user data remains
available through `XML_GetUserData`. `XML_SetUserData` updates the normal argument
when it still tracks the old user data. External entity references use their
own parser-or-override argument; unknown encoding uses its separately registered
handler data. These conventions must survive child parser creation.

Callbacks are synchronous and may change handler registrations or request stop.
That does not authorize arbitrary recursive parsing on the same parser, freeing
it during dispatch, or resuming it from inside a handler. External entity child
parsing is the supported nested parsing path. Use the public contracts and
specific dispatch sites to determine supported mutation points.

## Configuration timing and reset

Before parsing, set encoding override, namespace-triplet mode, PE policy,
foreign-DTD behavior, and hash salt. The common `parserBusy` implementation
considers PARSING and SUSPENDED busy, while INITIALIZED and FINISHED are not.
Public header wording generally requires these choices before input. For exact
edge-case compatibility, test finished/error-state setter calls as well as
ordinary before/during calls. A permitted setter call on a finished object does
not restart document parsing.

Handler changes and user/base data changes have their own timing semantics.
Reparse deferral can be changed during parsing, accepts only exact `XML_TRUE` or
`XML_FALSE`, and is copied into external children at creation. Resource-limit
setters require a root parser; they reject null/child objects and NaN or factors
below 1.0. They do not use the ordinary busy-state guard.

`XML_ParserReset` only works for a root parser. It clears document declarations,
handlers (except unknown-encoding registration), user data, input progress, base,
errors, encoding choice, entity work, and PE/foreign-DTD state. It releases an
active unknown decoder, but keeps its registered handler and handler data.
Namespace mode, separator, and triplet setting survive; many allocation
capacities/free lists survive for reuse.

Inspect `parserInit` and `parserCreate` separately for limits: expansion accounting
and its settings are reinitialized by reset, while allocation tracker fields
are initialized in construction and persist across reset. Tracked live cached
memory still exists. Hash salt resets to zero (automatic initialization on the
next root parse), and reparse deferral resets to its default. Do not assume
either all options survive or none do. Child parsers cannot be reset.

Fatal errors install `errorProcessor`; a reset is the supported way to begin
a new root document. After successful finalization, further parse/buffer calls
return `XML_ERROR_FINISHED`. Input/control functions have distinct checks for
negative sizes, missing buffer, null arguments, suspended state, and wrong resume
state; reproduce each path if aiming for drop-in API behavior.

## Public representation and binary compatibility

| Build choice | Representation |
| --- | --- |
| Default | `XML_Char=char` (UTF-8), `XML_LChar=char` |
| `XML_UNICODE` | `XML_Char=unsigned short` (UTF-16), `XML_LChar=char` |
| `XML_UNICODE_WCHAR_T` | `XML_Char` and `XML_LChar` are `wchar_t`; requires the supported 16-bit-wide setup and implies Unicode mode |
| Default positions | `XML_Index=long`, `XML_Size=unsigned long` |
| `XML_LARGE_SIZE` | `XML_Index=long long`, `XML_Size=unsigned long long` |

These are C type choices, not promises that `long` is always 32-bit. ABI
compatibility includes widths/alignment of `XML_Content`, `XML_Encoding`,
`XML_ParsingStatus`, `XML_Feature`, allocation-suite structs, optional attribute
info, callback calling conventions, and enum values. `XML_Bool` is an unsigned
byte, whereas several older callback parameters remain `int` for compatibility.

`XML_GetUserData` depends on the first parser field. `XMLPARSEAPI`, `XMLCALL`,
`XMLIMPORT`, `XML_STATIC`, and visibility macros control linkage/import/calling
conventions. `XML_LChar` applies to error/version strings and can differ from
the event-character type. Feature-dependent exports must match the
[`ELF map`](../expat/lib/libexpat.map.in) and
[`Windows definition`](../expat/lib/libexpat.def.cmake) templates for the build.
The header itself uses `expat_config.h`; distribute coherent generated config
and public headers.

## Build features

Defaults below are the standalone defaults expressed in
[`CMakeLists.txt`](../expat/CMakeLists.txt); a parent project/cache can override
them. Actual generated `expat_config.h` is the authority for a built library.

| CMake option | Core macro/default | Consequence |
| --- | --- | --- |
| `EXPAT_DTD` | `XML_DTD`, ON | Parameter entities, external subsets, conditional sections; requires GE |
| `EXPAT_GE` | `XML_GE`, ON | Normal general entity replacement, expansion accounting, allocation tracking; macro must be defined as numeric 0 or 1 |
| `EXPAT_NS` | `XML_NS`, ON | Namespace scanner tables/semantic support; runtime opt-in still matters |
| `EXPAT_CHAR_TYPE` | `char` default; `ushort`/`wchar_t` alternatives | Callback/output ABI and internal replacement encoding |
| `EXPAT_CONTEXT_BYTES` | `XML_CONTEXT_BYTES`, 1024 | Retained original-input context; zero enables direct-input path and disables context API output |
| `EXPAT_ATTR_INFO` | `XML_ATTR_INFO`, OFF | Extra attribute position workspace/API/export |
| `EXPAT_LARGE_SIZE` | `XML_LARGE_SIZE`, OFF | Position type ABI |
| `EXPAT_MIN_SIZE` | `XML_MIN_SIZE`, OFF | Less scanner specialization, smaller/slower implementation |
| `EXPAT_SHARED_LIBS` | Platform-dependent | Static/shared artifacts and import/export behavior |
| `EXPAT_BUILD_TESTS` | ON | C/C++ regression targets using testing internals |
| `EXPAT_BUILD_FUZZERS` | OFF | Fuzzer targets and optional external toolchain/dependency requirements |

The source requires a nonempty, nonnegative `XML_CONTEXT_BYTES`. Many boolean
features use `#ifdef`, so defining them to 0 is not necessarily equivalent to
undefining them. GE is explicitly different: it uses `#if XML_GE == 1`.
Endianness (`BYTEORDER`), entropy facilities, Windows helpers, compiler attributes,
and visibility definitions also come from configuration/platform detection.

The CMake setup rejects the built-in tests/examples/tools in certain `ushort`
configurations, and non-Windows `wchar_t` mode requires compatible 16-bit wchar
compiler and C-library support. A wide-character matrix needs an appropriate
toolchain, not just changing a typedef.

`XML_GetFeatureList` returns static entries ending in `XML_FEATURE_END`/null name.
Presence often indicates a boolean feature even when its value field is zero;
size/context/limit entries contain values. `XML_ExpatVersionInfo` returns the
numeric version triple; the version string also reflects this checkout's
versioning logic. A replacement should state its compatibility target and avoid
advertising absent features.
