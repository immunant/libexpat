# 6. Internal data structures

All of these are private to `xmlparse.c`. Every one of them allocates through
the parser (`MALLOC`/`REALLOC`/`FREE`), so the user's memory suite and the
allocation tracker apply.

## 6.1 Hash table (`HASH_TABLE`, `NAMED`)

```c
typedef const XML_Char *KEY;
typedef struct { KEY name; } NAMED;          // "base class" of every entry
typedef struct {
  NAMED **v;            // slot array, size = 2^power
  unsigned char power;
  size_t size;
  size_t used;
  XML_Parser parser;    // for allocation
} HASH_TABLE;
```

* **Intrusive, generic by layout.** Every entry type (`ENTITY`,
  `ELEMENT_TYPE`, `ATTRIBUTE_ID`, `PREFIX`) starts with `const XML_Char
  *name`. `lookup()` allocates `createSize` bytes, zeroes them, sets
  `name = key`, and returns a `NAMED *` that the caller casts. The table
  **does not copy the key**. The caller interns the name in a string pool
  first. After a lookup the caller checks `entry->name == name` to see
  whether the entry is new; if it is not, the caller discards its copy of the
  name with `poolDiscard`.
* **`lookup(parser, table, name, createSize)`** (`xmlparse.c:7835`):
  `createSize == 0` means find only. The table is created lazily with
  `INIT_POWER = 6` (64 slots).
* **Open addressing with double hashing.** The index is `h & mask`. On a
  collision the probe step is `PROBE_STEP(h, mask, power)`: hash bits that
  the mask threw away, limited to `size/4` and forced odd, so the step is
  coprime with the power-of-two size and the probe visits every slot. Probing
  moves backwards: `i < step ? i += size - step : i -= step`.
* **Growth:** when `used >> (power - 1)` is non-zero (the table is half full),
  the size doubles and everything is rehashed. There is no deletion; tables
  are only cleared or destroyed as a whole.
* **Hash function:** SipHash-2-4 over the key's bytes, with key
  `{0, m_hash_secret_salt}` taken from the **root** parser
  (`copy_salt_to_sipkey`). See [07](07-memory-and-security.md#73-hash-flooding-protection).
* **Iteration:** `HASH_TABLE_ITER { NAMED **p, **end; }` walks the slot array
  and skips empty slots. The order is unspecified. That affects, for example,
  the order of prefixes in `getContext()` strings.

`m_nsAtts` (the per-start-tag duplicate check for expanded attribute names)
is a **separate** small open-addressing table of `NS_ATT { version, hash,
uriName }`. It uses the same probe scheme and the version trick described in
[04 §4.9](04-parser-core.md#49-storeatts-attributes).

## 6.2 String pool (`STRING_POOL`, `BLOCK`)

```c
typedef struct block { struct block *next; int size; XML_Char s[]; } BLOCK;
typedef struct {
  BLOCK *blocks;       // in-use blocks; blocks->s holds the *current* string
  BLOCK *freeBlocks;   // blocks kept after poolClear, reused before malloc
  const XML_Char *end; // end of current block
  XML_Char *ptr;       // write position
  XML_Char *start;     // start of the string being built
  XML_Parser parser;
} STRING_POOL;
```

This is an **arena for building strings**. One string at a time is "open"
between `start` and `ptr`:

| Operation | Meaning |
|-----------|---------|
| `poolAppendChar(pool, c)` (macro) | append one `XML_Char`, growing if needed; returns 0 on OOM |
| `poolAppend(pool, enc, ptr, end)` | **transcode** bytes from `enc` into the pool (loops on `poolGrow` while the output is full) |
| `poolStoreString(...)` | `poolAppend` + NUL terminator; returns `start` |
| `poolCopyString(s)`, `poolCopyStringN(s, n)` | copy an `XML_Char` string and **finish** it |
| `poolAppendString(s)` | append without finishing |
| `poolFinish(pool)` | `start = ptr`: **commit** the current string; it stays valid until the pool is cleared |
| `poolDiscard(pool)` | `ptr = start`: throw away the open string |
| `poolChop`, `poolLastChar`, `poolLength`, `poolStart` | small accessors |
| `poolClear(pool)` | move all blocks to `freeBlocks`; all strings become invalid |
| `poolDestroy(pool)` | free all blocks |

`poolGrow` (`xmlparse.c:8114`) works like this:

1. If there is a free block big enough for the open string, move the string
   there (`memcpy`).
2. Else, if the open string *starts at the beginning* of the current block,
   **`realloc`** that block to twice its size. This is safe because no
   finished string lives in this block before `start`.
3. Else, allocate a new block of `max(INIT_BLOCK_SIZE (1024), 2 × current
   string length)` and copy the open string into it.

So a finished string never moves, but **the open string can move** each time
the pool grows. Code that holds pointers into an unfinished string must
re-read `poolStart()` after appending.

Pools used by the parser:

| Pool | Lifetime | Contents |
|------|----------|----------|
| `dtd->pool` | the DTD | element, attribute, entity, and prefix names; system/public IDs; attribute default values; `XML_SetBase` strings |
| `dtd->entityValuePool` | the DTD | replacement text of internal entities |
| `m_tempPool` | cleared after each start tag / PI / comment / declaration | attribute values for the start handler, expanded attribute names, empty-element names, doctype strings, `getContext` output |
| `m_temp2Pool` | short | entity names looked up while expanding attribute values, XML-declaration strings |

## 6.3 Element stack (`TAG`, `TAG_NAME`)

```c
typedef struct {
  const XML_Char *str;       // name passed to handlers (maybe "uri<sep>local[<sep>prefix]")
  const XML_Char *localPart; // NS: points into buf (after "prefix:")
  const XML_Char *prefix;    // NS: binding->prefix->name
  int strLen, uriLen, prefixLen;
} TAG_NAME;

typedef struct tag {
  struct tag *parent;          // the stack link; also the free-list link
  const char *rawName;         // document-encoding bytes (first in input buffer, later in buf)
  int rawNameLength;
  TAG_NAME name;
  union { char *raw; XML_Char *str; } buf;   // [ converted name \0 ][ raw name bytes ]
  char *bufEnd;
  BINDING *bindings;           // namespace declarations made on this element
} TAG;
```

* `m_tagStack` is a singly linked stack through `parent`. Popped tags go to
  `m_freeTagList` (through the same `parent` field) and are reused with
  their buffers, so a long document with a bounded depth stops allocating
  after warm-up.
* `buf` starts at `INIT_TAG_BUF_SIZE` (32 bytes) and doubles as needed.
* `m_tagLevel` is the depth, which `doContent` compares with
  `startTagLevel`.

## 6.4 Namespace bindings (`PREFIX`, `BINDING`)

```c
typedef struct prefix { const XML_Char *name; BINDING *binding; } PREFIX;

typedef struct binding {
  struct prefix *prefix;
  struct binding *nextTagBinding;    // list of bindings declared on one element
  struct binding *prevPrefixBinding; // previous (outer) binding of the same prefix
  const struct attribute_id *attId;  // the xmlns attribute, or NULL (inherited/implicit)
  XML_Char *uri;                     // "URI<sep>" + room for a local name and prefix
  int uriLen;                        // includes the separator if any
  int uriAlloc;
} BINDING;
```

Two linked structures are threaded through the same nodes:

```
             PREFIX "a".binding ──► B3 (inner a=...)  ──prevPrefixBinding──► B1 (outer a=...) ──► NULL
TAG <x xmlns:a=... xmlns:b=...>.bindings ──► B2(b) ──nextTagBinding──► B1(a) ──► NULL
TAG <y xmlns:a=...>.bindings             ──► B3(a) ──► NULL
```

* **Declare** (`addBinding`): `b->prevPrefixBinding = prefix->binding;
  prefix->binding = b; b->nextTagBinding = tag->bindings; tag->bindings = b`.
* **Undeclare at the end tag**: for each `b` in `tag->bindings`:
  `prefix->binding = b->prevPrefixBinding`, then move `b` to
  `m_freeBindingList`. Its `uri` buffer is kept and reused.
* The default namespace is the `PREFIX` `dtd->defaultPrefix` (name NULL).
  `xmlns=""` sets its binding to NULL while the `BINDING` record is still
  pushed, so the undeclaration is undone properly at the end tag.
* `m_inheritedBindings` holds bindings that come from `setContext()`: the
  implicit `xml` binding in the root parser, or the parent's bindings in an
  external-entity child.

## 6.5 Attribute structures

```c
typedef struct attribute_id {   // interned per qualified name in dtd->attributeIds
  XML_Char *name;               // name[-1] is a scratch flag (see 6.6)
  PREFIX *prefix;               // NS: prefix ("xmlns:p" -> PREFIX p; "xmlns" -> defaultPrefix)
  XML_Bool maybeTokenized;      // declared non-CDATA somewhere
  XML_Bool xmlns;               // is a namespace declaration
} ATTRIBUTE_ID;

typedef struct { const ATTRIBUTE_ID *id; XML_Bool isCdata; const XML_Char *value; }
  DEFAULT_ATTRIBUTE;

typedef struct {
  const XML_Char *name;
  PREFIX *prefix;
  const ATTRIBUTE_ID *idAtt;        // first ID-typed attribute
  int nDefaultAtts, allocDefaultAtts;
  DEFAULT_ATTRIBUTE *defaultAtts;   // ALL declared attributes (value NULL if #IMPLIED/#REQUIRED)
} ELEMENT_TYPE;
```

Note that `defaultAtts` holds **every declared attribute**, not only those
with defaults. It is also how `storeAtts` finds an attribute's declared type
(`isCdata`) for normalization.

`ATTRIBUTE` (from the tokenizer) is the raw view: `{name, valuePtr, valueEnd,
normalized}` pointing into the input. `m_atts` is an array of these, and it
is reused as the `const XML_Char **` array passed to the start handler.

## 6.6 The `name[-1]` flag byte on attribute IDs

`getAttributeId` appends one extra `XML_Char` (0) to `dtd->pool` **before**
the attribute name and then returns `name = start + 1`. So every interned
attribute name owns the slot just before it. `storeAtts` uses that slot as a
per-tag mark, with no extra memory:

| `name[-1]` | Meaning during one `storeAtts` call |
|-----------:|-------------------------------------|
| 0 | not seen on this tag |
| 1 | specified or defaulted, no namespace expansion needed |
| 2 | prefixed and needs namespace expansion |

Every flag is reset to 0 before `storeAtts` returns, including for `xmlns`
attributes through the tag's binding list. `dtdCopy` makes sure the copied
names also get this leading slot.

## 6.7 Free lists and reuse

Expat avoids `malloc` in steady state by recycling:

| Object | In use | Free list |
|--------|--------|-----------|
| `TAG` (+ `buf`) | `m_tagStack` | `m_freeTagList` |
| `BINDING` (+ `uri`) | `tag->bindings`, `prefix->binding`, `m_inheritedBindings` | `m_freeBindingList` |
| `OPEN_INTERNAL_ENTITY` ×3 | `m_open{Internal,Attribute,Value}Entities` | `m_free{Internal,Attribute,Value}Entities` |
| pool `BLOCK`s | `pool->blocks` | `pool->freeBlocks` |
| `m_atts`, `m_attInfo`, `m_nsAtts`, `m_dataBuf`, `m_groupConnector` | grow-only arrays owned by the parser | — |

`XML_ParserReset` moves everything in use to the free lists instead of
freeing it. The DTD's hash tables are emptied (entries freed), but the slot
arrays are kept.

## 6.8 Ownership summary

```
XML_ParserStruct
├── m_buffer                 (raw malloc_fcn; NOT tracked)
├── m_dataBuf, m_atts, m_attInfo, m_nsAtts, m_groupConnector
├── m_tempPool, m_temp2Pool  ─► BLOCKs
├── m_tagStack / m_freeTagList ─► TAG ─► buf; bindings ─► BINDING ─► uri
├── m_freeBindingList, m_inheritedBindings ─► BINDING
├── m_open*/m_free* entity lists ─► OPEN_INTERNAL_ENTITY (point at ENTITYs in the DTD)
├── m_protocolEncodingName (copyString)
├── m_unknownEncodingMem (+ user's data, released via m_unknownEncodingRelease)
└── m_dtd (owned unless m_isParamEntity) ─► DTD
        ├── 4–5 HASH_TABLEs ─► v[] ─► entries (ENTITY / ELEMENT_TYPE / ATTRIBUTE_ID / PREFIX)
        │                                  └── ELEMENT_TYPE.defaultAtts[]
        ├── pool, entityValuePool ─► BLOCKs (all names and values live here)
        └── scaffold[], scaffIndex[]   (root parser only)
```

Strings given to handlers point into the input buffer, `m_dataBuf`,
`m_tempPool`, `dtd->pool`, or a binding's URI buffer. They are valid **only
during the callback**, except for the `XML_Content` model, which the
application owns.
