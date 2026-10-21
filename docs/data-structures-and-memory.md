# Expat data structures and memory

[Guide index](README.md)

Most semantic structures are private to [`xmlparse.c`](../expat/lib/xmlparse.c). They hold the minimum state needed to resume parsing and deliver events, plus reusable allocations. The token-layer structures are in [`xmltok.h`](../expat/lib/xmltok.h), and the public content-model tree is in [`expat.h`](../expat/lib/expat.h).

## Parser state by responsibility

| State | Important members | Invariant |
| --- | --- | --- |
| Application | `m_userData`, `m_handlerArg`, handlers, external-handler argument | Handler arguments are configurable and are not all identical |
| Input | `m_buffer`, `m_bufferPtr`, `m_bufferEnd`, `m_bufferLim` | Pending bytes remain owned and addressable between parse calls |
| Encoding | `m_encoding`, `m_initEncoding`, `m_internalEncoding`, protocol encoding, unknown-encoding state | Source encoding and stored replacement-text encoding are separate |
| Control | `m_processor`, `m_prologState`, `m_parsingStatus`, `m_errorCode`, `m_reenter` | Grammatical continuation, lifecycle state, and error state are separate |
| Positions | `m_eventPtr`, `m_eventEndPtr`, `m_positionPtr`, `m_position`, `m_parseEndByteIndex`, `m_parseEndPtr` | Reported locations remain in source bytes/characters rather than callback-buffer offsets |
| Elements | `m_tagStack`, `m_tagLevel`, `m_freeTagList` | Open elements form a stack; reusable tags are separate |
| Namespaces | `m_inheritedBindings`, `m_freeBindingList`, namespace separator/triplet settings | Prefix bindings restore the previous scope when their element ends |
| Entities | Three open and three free entity-frame lists | Content, attribute, and declaration-value expansion retain their own continuations |
| DTD | `m_dtd`, current declaration fields | Declarations persist beyond the token that introduced them |
| Attributes | `m_atts`, `m_nsAtts`, specified count, ID index, optional position array | Start-element arguments are fully assembled before delivery |
| Temporary strings | `m_tempPool`, `m_temp2Pool`, `m_dataBuf` | Scratch data must not escape its defined lifetime |
| Family and limits | `m_parentParser`, hash salt, accounting, allocation tracker | Child activity is charged to the root's resource state |

The first member of `XML_ParserStruct` must remain `m_userData` for C ABI compatibility: `XML_GetUserData` is a macro that reads the first pointer-sized field of the opaque parser handle.

## Elements and namespace bindings

`TAG` contains a parent pointer, the raw name and byte length, a `TAG_NAME` in the API encoding, a growable name buffer, and a list of namespace bindings introduced on the element. Raw names support exact end-tag matching. Converted names support callback delivery.

Raw names initially borrow bytes from the parse input. `storeRawNames` copies the names of still-open tags before those bytes can become invalid. `TAG_NAME.localPart` and related metadata allow expanded names to be reconstructed later. In namespace mode, `TAG_NAME.str` can point into a binding's URI buffer rather than its own tag buffer; resizing that URI buffer updates affected open-tag pointers.

`PREFIX` stores an interned prefix name and its current `BINDING`. Each binding has two independent links:

- `prevPrefixBinding` restores the earlier binding of that prefix.
- `nextTagBinding` links declarations introduced by the same tag, or links a free/inherited list as appropriate.

It also owns a URI buffer, records its prefix, and refers to the declaring `ATTRIBUTE_ID`. The default namespace uses `DTD.defaultPrefix` with a null name. This stack-of-bindings model supports shadowing without copying the entire namespace environment for every element.

## DTD objects and identity

`DTD` owns hash tables for general entities, element types, attribute identifiers, prefixes, and, with `XML_DTD`, parameter entities. It has a long-lived string pool and a separate entity-value pool, plus standalone/parameter-reference flags and content-model scaffolding.

| Object | Stored information |
| --- | --- |
| `ENTITY` | Name; internal replacement text and length or external IDs; base and notation; open/continuation flags; declaration provenance |
| `ELEMENT_TYPE` | Name and optional prefix; ID attribute identity; array of default/type metadata for attributes |
| `ATTRIBUTE_ID` | Interned lexical name, prefix, namespace-declaration flag, and possible tokenized-type flag |
| `DEFAULT_ATTRIBUTE` | Attribute identity, whether the type is CDATA, and an optional normalized default value |
| `CONTENT_SCAFFOLD` | Content-model type/quantifier/name and child/sibling indexes used while a declaration is assembled |

Names are interned by lexical spelling, not namespace-expanded identity. DTD declarations apply to lexical element/attribute names. `ATTRIBUTE_ID.name[-1]` is a deliberately allocated extra character used as a per-start-tag marker: absent, present, or present and awaiting prefix expansion. Never port that indexing trick without also allocating its prefix slot. A separate marker field or per-tag set is a clearer alternative.

The same attribute name can have different types on different element declarations. `maybeTokenized` is a hint to consult the current element's metadata; it is not itself the attribute's definitive type.

## Entity frames

`OPEN_INTERNAL_ENTITY` links a live expansion to its `ENTITY`, records entry tag depth and whether a parameter reference was between declarations, and supplies internal event pointers. `ENTITY.processed` is a **byte offset**, while `ENTITY.textLen` is in **XML_Char units**. Mixing those units breaks UTF-16 continuation.

`ENTITY.open` is cycle-detection state. `ENTITY.hasMore` means its own replacement text has more work. An entity can have `hasMore == false` while still open because an inner entity has not finished. Closing it early would let indirect recursion evade detection. Each completed frame returns to its corresponding free list.

## Hash tables

`HASH_TABLE` is an open-addressed array of pointers to objects whose first field is a name, represented generically by `NAMED`. `lookup(..., createSize=0)` only searches; a nonzero size allocates a zero-initialized record if absent. The supplied name pointer is stored rather than copied, so the caller must commit it to stable storage when creating an entry.

Capacity is a power of two. Lookup masks the hash to get an index, then uses an odd double-hashing probe step on collisions. Growth occurs around half occupancy and moves pointers, preserving object addresses. Clear frees the entries but retains the bucket array; destroy also frees the array.

SipHash uses API-encoding name bytes. `copy_salt_to_sipkey` puts zero in the first 64-bit key word and the root parser's `unsigned long` salt in the second. It is not a newly generated independent 128-bit key per table. Hash and allocation ownership are separate arguments to `lookup`, which matters while copying DTDs into child parsers.

`NS_ATT` is a separate temporary hash table for expanded attribute names. A generation/version field avoids clearing the entire array for every tag; wraparound forces reinitialization. Duplicate detection compares namespace URI plus local name, excluding the original prefix even when triplet output is enabled.

## String pools

`STRING_POOL` manages linked `BLOCK` allocations containing `XML_Char` arrays. `start..ptr` is the current uncommitted string; `end` is available capacity. Earlier committed strings stay valid until clear/destroy.

| Operation | Meaning |
| --- | --- |
| `poolAppend`, `poolAppendChar` | Extend the current string, growing as necessary |
| `poolStoreString` | Convert a source range and append a terminator; callers decide whether to commit |
| `poolFinish` | Commit the current region by moving `start` to `ptr` |
| `poolDiscard` | Abandon the current string by resetting `ptr` to `start` |
| `poolClear` | Invalidate all strings and move blocks to a reusable free list |
| `poolDestroy` | Release active and free blocks |

Growth may reallocate the current block when it contains only the current uncommitted string, or copy that unfinished string to another block. It must not move already committed strings. A caller holding a pointer into an unfinished string must therefore account for growth.

`DTD.pool` retains names, default values, and identifiers. `DTD.entityValuePool` retains replacement text. Parser temporary pools are cleared after events or declaration work, and `m_dataBuf` is reused during conversion. Stable internal identifiers do not imply that arbitrary callback strings are safe for applications to retain.

## Ownership boundaries

| Data | Owner and usable lifetime |
| --- | --- |
| Input passed to `XML_Parse` | Application; Expat copies pending bytes before returning when needed |
| Storage returned by `XML_GetBuffer` | Parser; treat later parser operations as potentially invalidating the pointer |
| Element names, attribute arrays/values, comment and PI strings | Borrowed during the callback; copy to retain |
| Character/default-handler data | Borrowed range with explicit length, not necessarily null-terminated |
| Entity declaration value | Borrowed range with explicit length; zero length still denotes an internal entity when pointer is non-null |
| External entity context | Borrowed until the external-handler return; copy it for deferred use |
| `XML_Content` from element-declaration callback | Ownership transferred to application; release with `XML_FreeContentModel` |
| Unknown-encoding `data` | Application-defined state with a release callback that Expat invokes when finished |
| Base returned by `XML_GetBase` | Parser-owned stored base; invalidated when replaced/reset/freed |

## Allocation and destruction

The memory suite supplied to `XML_ParserCreate_MM` governs further memory operations. With `XML_GE=1`, most internal allocations pass through tracked wrappers, which prepend size metadata and maintain root-level accounting. Input buffers, application allocations through `XML_Mem*`, and exported content models deliberately use the raw suite instead. Mixing the raw and tracked free paths corrupts allocation metadata.

`XML_ParserFree` handles both live and free lists, scratch buffers, pools, unknown-encoding release, and DTD ownership. External parameter parsers borrow the DTD and must not destroy it; general-entity parsers own copied DTD state. Keep ancestors alive until their children are freed because accounting and hash access follow `m_parentParser`.

`XML_ParserReset` is root-only. It moves reusable tags, bindings, and entity frames to free lists, clears temporary and DTD contents, releases encoding state, and reinitializes parsing. It preserves selected configuration and reusable storage; it is not equivalent to freeing and allocating a new object in every detail. See [API compatibility](api-compatibility.md).

For a rewrite, explicit owners, stable handles, and integer buffer offsets can replace these pointer arrangements. Preserve lifetimes and callback behavior while checking that moved buffers, shared DTDs, failure cleanup, and reset remain correct.
