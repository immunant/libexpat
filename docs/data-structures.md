# State, storage, and ownership

The structures in this chapter are defined in
[`xmlparse.c`](../expat/lib/xmlparse.c), except the scanner structures in
[`xmltok.h`](../expat/lib/xmltok.h), grammar state in
[`xmlrole.h`](../expat/lib/xmlrole.h), and public structures in
[`expat.h`](../expat/lib/expat.h).

## Parser object

`XML_Parser` is an opaque pointer to `XML_ParserStruct`. Its fields fall into
the following working sets:

| Fields | Purpose |
| --- | --- |
| `m_userData`, `m_handlerArg`, `m_externalEntityRefHandlerArg` | Application state and the distinct callback argument conventions |
| `m_mem` | Copied allocation suite; immutable for the parser's lifetime |
| `m_buffer*`, `m_parseEnd*` | Owned input span, cursor, capacity, and stream-byte offset basis |
| `m_partialTokenBytesBefore`, `m_lastBufferRequestSize`, `m_reparseDeferralEnabled` | Avoid repeated scanning of large unfinished tokens |
| `m_encoding`, `m_initEncoding`, `m_internalEncoding`, `m_protocolEncodingName` | Document decoding, initial auto-detection state, replacement-text decoding, protocol override |
| `m_unknownEncoding*` | Decoder allocation, application data, release callback, and handler registration |
| Handler fields | Current event subscriptions; application callbacks can change handlers |
| `m_processor`, `m_prologState`, `m_reenter` | Semantic phase, DTD grammar state, and iterative processor handoff |
| `m_errorCode`, `m_parsingStatus` | Last error and public lifecycle/final-input status |
| `m_eventPtr`, `m_eventEndPtr`, `m_positionPtr`, `m_position` | Original-input event range and lazily updated line/column basis |
| `m_tagStack`, `m_tagLevel`, `m_freeTagList` | Open elements and recyclable frames |
| Three open/free entity-list pairs | Separate continuations for content/prolog, attribute expansion, and declaration replacement-value expansion |
| `m_dtd`, `m_decl*`, `m_doctype*`, `m_curBase` | Persistent declarations and the declaration currently being assembled |
| `m_atts`, `m_attsSize`, `m_nSpecifiedAtts`, `m_idAttIndex` | Raw attribute workspace reused for callback name/value pointers and metadata |
| `m_nsAtts*`, `m_ns*`, binding lists | Namespace options, duplicate-expanded-attribute workspace, inherited/recycled bindings |
| `m_tempPool`, `m_temp2Pool`, `m_dataBuf*` | Event-local strings, nested normalization scratch, bounded conversion output |
| `m_groupConnector`, `m_groupSize` | DTD model-group connector consistency by nesting level |
| `m_parentParser` | External parser ancestry; also the route to shared limits and hash secret |
| `m_accounting`, `m_alloc_tracker`, `m_entity_stats` | Root expansion/allocation budgets and diagnostics when `XML_GE == 1` |

The **first member is `m_userData`**. `XML_GetUserData(parser)` is a public macro
that dereferences the parser as a pointer to a pointer. A binary-compatible
opaque implementation cannot arbitrarily move that first member.

## Input buffer and position model

```text
m_buffer                      m_bufferPtr          m_bufferEnd     m_bufferLim
    | retained consumed context | unconsumed bytes     | free capacity |
```

`m_bufferPtr` is the next document byte to process; `m_bufferEnd` is the end of
committed input. `XML_GetBuffer(n)` returns space beginning at the latter,
preserving any unconsumed bytes and optionally a prefix of consumed context.
Compaction or replacement changes pointer addresses without changing stream
offsets. `m_parseEndByteIndex` and `m_parseEndPtr` relate event pointers to
absolute byte positions.

The buffer belongs to the parser. `XML_Parse` may process application bytes
directly only in the zero-context build and when there is no buffered remainder.
An unfinished suffix must be copied before returning. Raw names of still-open
tags must also survive reuse of the input memory; see `storeRawNames`.

`POSITION` stores zero-based internal line and column numbers. Public lines add
one; public columns remain zero-based. Byte offsets measure encoded source
bytes, not UTF-8 output length or Unicode scalar count.

## Open elements and namespace bindings

`TAG` is a linked stack frame with `parent`, `rawName`, `rawNameLength`, a
converted `TAG_NAME`, an owned buffer, and a list of bindings introduced by
that element. The original spelling is used for exact end-tag matching.
Converted names are for callback output. Closed frames move to `m_freeTagList`.

`TAG_NAME` tracks `str`, `localPart`, `prefix`, and their lengths. In namespace
mode, the callback element name can point into a binding's mutable URI buffer.
Nested elements can overwrite its appended local part. Therefore end-tag
reporting reconstructs local/prefix components from the tag frame; expanding a
binding buffer must repair all tag pointers that referred to the old buffer.

`PREFIX` has an interned name and a pointer to its current `BINDING`.
The default namespace uses `DTD.defaultPrefix`, whose name is null.
A `BINDING` contains:

- A prefix and the `prevPrefixBinding` to restore on scope exit.
- `nextTagBinding`, linking all declarations introduced by an element.
- The declaring `ATTRIBUTE_ID`, or null for inherited/implicit context.
- A mutable URI buffer, used length, and capacity; the stored URI length
  includes a nonzero namespace separator.

This creates two simultaneous chains: bindings belonging to one tag, and older
bindings shadowed for one prefix. Pop the former and restore the latter. A
replacement can use immutable expanded names and a stack of maps, but must
preserve event order and restoration behavior.

## DTD tables

`DTD` holds hash tables of `ENTITY`, `ELEMENT_TYPE`, `ATTRIBUTE_ID`, and `PREFIX`
records, plus a parameter-entity table when `XML_DTD` is enabled. Its two pools
hold general persistent strings and entity replacement text. It also owns
declaration-processing flags and content-model scaffolding.

| Record | Significant fields and meaning |
| --- | --- |
| `ENTITY` | Name; replacement `textPtr`/`textLen` or external IDs; base; notation; `open` recursion guard; `processed` byte offset; `hasMore`; general/parameter and declaration-origin flags |
| `ELEMENT_TYPE` | Interned lexical name, namespace prefix, ID attribute, and growable `DEFAULT_ATTRIBUTE` array |
| `ATTRIBUTE_ID` | Interned lexical name, prefix, `xmlns` marker, and `maybeTokenized` hint |
| `DEFAULT_ATTRIBUTE` | Attribute identity, CDATA/non-CDATA normalization flag, and optional normalized default |
| `NS_ATT` | Generation/version marker, expanded-name hash, expanded-name pointer for duplicate detection within one start tag |
| `CONTENT_SCAFFOLD` | Content type and quantifier, optional name, child/sibling indices and child count |

The term `DEFAULT_ATTRIBUTE` includes declarations without an actual default:
their type and ID metadata still matter. `maybeTokenized` is only a global
hint; the current element's declaration determines whether this use is CDATA.

The character **before** `ATTRIBUTE_ID.name` is reserved writable scratch:
zero means absent, one means encountered, and two denotes a prefixed attribute
awaiting expansion. Allocation and `dtdCopy` must preserve that extra slot.
Successfully completing `storeAtts` clears the markers. Fatal error cleanup can
leave them set because that document will not continue normally.

## Entity continuations

`OPEN_INTERNAL_ENTITY` is a reusable linked frame containing the referenced
`ENTITY`, its expansion category, starting tag depth, `betweenDecl`, and
internal event pointers. The three categories are `ENTITY_INTERNAL`,
`ENTITY_ATTRIBUTE`, and `ENTITY_VALUE`.

`textLen` counts `XML_Char` units; `processed` counts **bytes**, including in
Unicode builds. Confusing those units corrupts resumed offsets. An entity can
be `open == true` while `hasMore == false`: its own replacement text has ended,
but a nested entity scheduled at the end still must finish. Clearing `open`
too soon defeats cycle detection. The active lists preserve this distinction
without native recursion.

## String pools

`STRING_POOL` owns active `BLOCK`s and a free-block cache. Each `BLOCK` has a
capacity in `XML_Char` units and a flexible character array. The current string
occupies `[start, ptr)` inside the current block, with `end` marking capacity.

| Operation | Meaning |
| --- | --- |
| `poolAppend*` / `poolStoreString` | Extend the current unfinished string; conversion occurs as necessary; `poolStoreString` adds NUL |
| `poolFinish` | Commit the current string by advancing `start` to `ptr`; its address remains valid while the pool lives and is not cleared |
| `poolDiscard` | Rewind `ptr` to `start`; discard only the unfinished string |
| `poolClear` | Invalidate all strings and move blocks to reusable storage |
| `poolDestroy` | Free active and cached blocks |
| `poolGrow` | Move/grow only the unfinished string; preserve previously committed strings |

`poolGrow` can reallocate a block only when the unfinished string starts at the
beginning of that block. Otherwise it allocates another block and copies the
unfinished string. This is what keeps already-published interned strings stable.
Temporary pools clear frequently; DTD pools live for a document/reset cycle.

## Interning and hash layout

`KEY` is a NUL-terminated `XML_Char *`. `NAMED` consists of the name field; all
hashable record types begin with that same field. `HASH_TABLE` stores pointers
to these records, its size/power/occupancy, and its owning parser for allocation.

`lookup(parser, table, name, createSize)` searches when `createSize == 0`, or
creates a zero-initialized record otherwise. A new record adopts the supplied
name pointer; it does not copy the name. Callers distinguish insertion from
an existing record by checking whether the returned name pointer is the newly
built string, then finish or discard the pool string accordingly.

The initial capacity is 64, capacities are powers of two, and growth occurs
around half occupancy. Collisions use an odd double-hashing step derived from
other hash bits. Moving the slot array does not move records. SipHash hashes
the `XML_Char` bytes; all related parsers use the root salt. A replacement may
use another hardened map if internal hashes/iteration order are not part of its
chosen compatibility target.

## Allocation and lifetime boundaries

| Storage | Owner and validity |
| --- | --- |
| Caller input to `XML_Parse` | Caller owns it; implementation must preserve anything needed after return |
| `XML_GetBuffer` result | Parser owns it; invalidated by operations that move/grow/free its buffer |
| Callback strings, attribute arrays, character spans | Borrowed; copy what the application needs to retain; character spans are length-delimited |
| DTD records and interned strings | Parser's DTD/reset lifetime, with special parent/child sharing rules |
| Tag/entity/binding free lists | Parser lifetime, often reused after reset |
| `XML_Content` delivered to element-declaration handler | Ownership transfers to application; free through `XML_FreeContentModel` with the appropriate allocation suite |
| Unknown-encoding application data | Released through the accepted encoding's release callback on reset/free; rejection paths also release supplied data |
| External child parser | Application creates and frees it; free children before their ancestor to preserve ancestry-based accounting and shared ownership |

Most internal allocations go through tracked `MALLOC/REALLOC/FREE` when general
entities are enabled. Input buffers, application-owned content models, and the
public memory helpers deliberately use the raw allocation suite. Do not mix
tracked allocation pointers with raw frees. See
[resource accounting](security-and-performance.md).

External general-entity parsers copy declaration records, strings, and defaults
and restore open-entity/namespace context. `dtdCopy` deliberately shallow-copies
content-model scaffolding; this is not a universal deep clone of every field.
External parameter-entity parsers share the DTD itself and must run synchronously.
Do not infer unrestricted lifetime or concurrent access from the copied tables.
