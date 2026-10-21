# 5. DTD, entities, and external entities

Expat is **non-validating**. Even so, it processes the DTD for everything
that affects the document's *content*:

* internal and external general entities,
* parameter entities (with `XML_DTD`),
* attribute defaults and attribute types (which control normalization and
  ID detection), and
* the standalone declaration.

It also reports all declarations through handlers. Content models are parsed
and passed to the application, but they are **never checked against the
document**.

## 5.1 The `DTD` object

`xmlparse.c:393`:

```c
typedef struct {
  HASH_TABLE generalEntities;   // name -> ENTITY
  HASH_TABLE elementTypes;      // name -> ELEMENT_TYPE
  HASH_TABLE attributeIds;      // name -> ATTRIBUTE_ID   (shared across elements!)
  HASH_TABLE prefixes;          // name -> PREFIX          (namespaces)
  STRING_POOL pool;             // names, attribute default values, sysids, ...
  STRING_POOL entityValuePool;  // replacement text of internal entities
  XML_Bool keepProcessing;      // false once a PE reference was skipped (and !standalone)
  XML_Bool hasParamEntityRefs;  // seen any PE reference or an external subset
  XML_Bool standalone;          // standalone="yes"
#ifdef XML_DTD
  XML_Bool paramEntityRead;     // the last external PE handler actually parsed it
  HASH_TABLE paramEntities;     // name -> ENTITY ("#" = the external subset)
#endif
  PREFIX defaultPrefix;         // the default namespace (name == NULL)
  /* content-model scaffolding */
  XML_Bool in_eldecl;
  CONTENT_SCAFFOLD *scaffold;
  unsigned contentStringLen, scaffSize, scaffCount;
  int scaffLevel;
  int *scaffIndex;
} DTD;
```

The DTD is a separate heap object because it is **shared or copied between
parsers**:

* An **external parameter entity parser** (or external subset parser)
  *shares* its parent's `DTD` pointer, so declarations it reads go straight
  into the document's DTD. `XML_ParserFree` does not destroy a shared DTD
  (the `m_isParamEntity` check).
* An **external general entity parser** gets a **deep copy** (`dtdCopy`,
  `xmlparse.c:7602`). It sees all declarations, but its changes stay private.
  The copy is made with the parent's hash salt (lookups go through
  `oldParser`). The content-model scaffolding is copied *shallowly*, and only
  the root parser's `dtdDestroy` frees it (`isDocEntity`).

Things to know about the tables:

* `attributeIds` is keyed by the attribute's **qualified name**, independent
  of element. An `ATTRIBUTE_ID` therefore stores facts that hold for every
  element: its `PREFIX`, whether it is an `xmlns` declaration, and
  `maybeTokenized` (whether *any* element declares it with a non-CDATA type).
  Facts that depend on the element, such as type and default value, live in
  `ELEMENT_TYPE.defaultAtts[]`.
* `elementTypes` entries are created on demand when an element is *used*
  (in `storeAtts`), not only when one is declared.
* The predefined entities (`lt`, `gt`, `amp`, `quot`, `apos`) are **never**
  in `generalEntities`. They are recognized by the tokenizer
  (`XmlPredefinedEntityName`). Declarations of them are silently ignored
  (`XML_ROLE_GENERAL_ENTITY_NAME` sets `m_declEntity = NULL`).
* **The first declaration of an entity wins** (XML 1.0 §4.2). `lookup()`
  returns the existing entry, the parser sees `entity->name != name`,
  discards the new name, and sets `m_declEntity = NULL`, so the rest of the
  declaration is parsed and then ignored. The same applies to the first
  attribute definition per (element, attribute) pair (`defineAttribute`
  returns early if it finds a duplicate).

## 5.2 `ENTITY`

```c
typedef struct {
  const XML_Char *name;
  const XML_Char *textPtr;   // replacement text (internal entities), in XML_Char
  int textLen;               // in XML_Chars
  int processed;             // how far expansion has got (for suspend / re-enter)
  const XML_Char *systemId;  // external entities
  const XML_Char *base;      // value of XML_SetBase at declaration time
  const XML_Char *publicId;  // normalized
  const XML_Char *notation;  // unparsed (NDATA) entities
  XML_Bool open;             // currently being expanded -> recursion detection
  XML_Bool hasMore;          // text not fully tokenized yet
  XML_Bool is_param;
  XML_Bool is_internal;      // declared in the internal subset, outside any PE
} ENTITY;
```

How to tell the kinds apart:

| Kind | Test |
|------|------|
| Internal (parsed) | `textPtr != NULL` |
| External parsed | `systemId != NULL && notation == NULL` |
| Unparsed | `notation != NULL` (referencing one with `&name;` is `XML_ERROR_BINARY_ENTITY_REF`) |

`is_internal` is set to `!(m_parentParser || m_openInternalEntities)` at
declaration time. It exists for the well-formedness constraint
*Entity Declared*: in a standalone document, or one without external markup,
a reference to an entity that was declared externally is an error
(`XML_ERROR_ENTITY_DECLARED_IN_PE`).

## 5.3 Processing declarations (roles → actions)

All of this happens in `doProlog`. Unless noted, the action runs only if
`dtd->keepProcessing` is true.

### Entities

| Role | Action |
|------|--------|
| `GENERAL_ENTITY_NAME` / `PARAM_ENTITY_NAME` | Intern the name. `lookup(..., sizeof(ENTITY))` creates the entry; if it already existed, ignore this declaration (`m_declEntity = NULL`). |
| `ENTITY_VALUE` | `callStoreEntityValue` decodes the literal into `dtd->entityValuePool`. The result becomes `textPtr`/`textLen`. Calls `EntityDeclHandler(name, is_param, value, len, base, NULL, NULL, NULL)`. |
| `ENTITY_SYSTEM_ID`, `ENTITY_PUBLIC_ID` | Store the IDs. A public ID is checked with `XmlIsPublicId` (`XML_ERROR_PUBLICID`) and normalized with `normalizePublicId`: runs of whitespace become one space, and leading/trailing whitespace is removed. `base = m_curBase`. |
| `ENTITY_NOTATION_NAME` | Unparsed entity: set `notation`. Calls `UnparsedEntityDeclHandler` if set, otherwise `EntityDeclHandler(..., notationName)`. |
| `ENTITY_COMPLETE` | End of an external entity declaration without NDATA: `EntityDeclHandler(name, is_param, NULL, 0, base, sysid, pubid, NULL)`. |

**Decoding an entity value** (`storeEntityValue`, `xmlparse.c:6772`):

* character references are expanded right away (`&#38;` becomes `&`);
* **general entity references are *not* expanded**. `&foo;` is stored
  literally and expanded later, wherever the entity is used (this is what the
  XML spec requires);
* newlines are normalized to LF;
* parameter-entity references are expanded **only** in external DTD content,
  or inside text that is already the replacement text of another PE. In the
  internal subset they give `XML_ERROR_PARAM_ENTITY_REF`. An internal PE is
  pushed with `processEntity(..., ENTITY_VALUE)` onto `m_openValueEntities`
  and drained by the loop in `callStoreEntityValue`. An external PE calls the
  external-entity handler while `m_prologState.inEntityValue` is set; the
  child parser then runs `entityValueInitProcessor`/`entityValueProcessor`
  and appends straight into the shared `entityValuePool`. A reference to the
  entity being declared, or to one that is open, gives
  `XML_ERROR_RECURSIVE_ENTITY_REF`.

With `XML_GE == 0` the value is not decoded. `storeSelfEntityValue` stores
the literal text `&amp;name;`, so wherever the entity is referenced the
application receives `&name;` as character data.

### Attribute lists

The role machine produces a sequence like
`ATTLIST_ELEMENT_NAME, ATTRIBUTE_NAME, ATTRIBUTE_TYPE_*, (ENUM_VALUE|NOTATION_VALUE)*, (IMPLIED|REQUIRED|DEFAULT|FIXED)_ATTRIBUTE_VALUE`
for each attribute:

* `m_declElementType = getElementType()` and `m_declAttributeId = getAttributeId()`.
* The type string is built in `m_tempPool`. For enumerations it is literally
  `"(a|b|c)"`; for notations, `"NOTATION(a|b)"`. Other types use a constant
  (`"CDATA"`, `"ID"`, `"IDREF"`, …).
* At the final role, `defineAttribute(type, attId, isCdata, isId, value)`
  appends a `DEFAULT_ATTRIBUTE` (value NULL for `#IMPLIED`/`#REQUIRED`). It
  records the first `ID`-typed attribute as `type->idAtt` (unless it is an
  `xmlns` attribute) and sets `attId->maybeTokenized` if the type is not
  CDATA. Then it calls
  `AttlistDeclHandler(elname, attname, att_type, dflt, isrequired)`.
  `isrequired` is 1 for `#REQUIRED` and also for `#FIXED`; `dflt` is NULL for
  `#IMPLIED`/`#REQUIRED`.
* A default value is normalized at declaration time with
  `storeAttributeValue` (into `dtd->pool`), which also expands entity
  references in it. `XML_ACCOUNT_NONE` is used because the bytes were already
  counted.

### Notations

`NOTATION_NAME`, then `NOTATION_PUBLIC_ID` and/or `NOTATION_SYSTEM_ID` or
`NOTATION_NO_SYSTEM_ID`, then
`NotationDeclHandler(name, base, systemId, publicId)`. Notations are not
stored in the DTD; they are only reported.

### Element declarations and content models

Content models are built only if an `ElementDeclHandler` is set
(`dtd->in_eldecl`).

* `ELEMENT_NAME` sets `scaffLevel = scaffCount = 0`.
* `CONTENT_ANY` / `CONTENT_EMPTY` report a single malloc'd `XML_Content`
  right away.
* `GROUP_OPEN` adds a scaffold node (type `SEQ` to start with) and pushes its
  index on `scaffIndex[]`.
* `GROUP_CHOICE` changes the current node to `CHOICE`, unless it is `MIXED`.
* `CONTENT_PCDATA` marks the current group `MIXED`.
* `CONTENT_ELEMENT(_OPT/_REP/_PLUS)` adds a `NAME` leaf with a quantifier.
  The name is interned through `getElementType`, and its length plus NUL is
  added to `contentStringLen`.
* `GROUP_CLOSE(_OPT/_REP/_PLUS)` sets the quantifier and pops the level. When
  the level reaches 0, `build_model()` produces the result and the handler
  gets it.

`CONTENT_SCAFFOLD` nodes are stored in a growing array and linked by index
(`firstchild`, `lastchild`, `childcnt`, `nextsib`). `build_model`
(`xmlparse.c:8291`) turns them into **one malloc'd block**:

```
[ XML_Content × scaffCount ][ name strings, NUL-terminated, back to back ]
```

It does this **iteratively, breadth-first**, using the `numchildren` field of
unfilled slots as a job queue of source indices. (The old recursive
`build_node` could exhaust the stack.) Children of a node are contiguous in
the array, so `children` can point into it. The block is allocated with plain
`m_mem.malloc_fcn`, so the application can free it with
`XML_FreeContentModel`, or with `free()` when using the default allocator.

The `,` / `|` consistency inside one group is checked with
`m_groupConnector[level]`, a byte per nesting level (0, `,`, or `|`).

### DOCTYPE

* `DOCTYPE_NAME`, `DOCTYPE_PUBLIC_ID`, `DOCTYPE_SYSTEM_ID` collect into
  `m_doctypeName/Pubid/Sysid`.
* `StartDoctypeDeclHandler(name, sysid, pubid, has_internal_subset)` is called
  at `DOCTYPE_INTERNAL_SUBSET` (with 1) or, if there is no internal subset,
  at `DOCTYPE_CLOSE` (with 0).
* At `DOCTYPE_CLOSE` with a system ID (or `XML_UseForeignDTD`), when
  `m_paramEntityParsing` and an external-entity handler are both set, the
  external subset is loaded: `ExternalEntityRefHandler(arg, context=NULL,
  base, sysid, pubid)`. It is stored as the parameter entity named `"#"`,
  which cannot clash with a real PE name. Then `EndDoctypeDeclHandler` is
  called.
* `XML_UseForeignDTD(TRUE)` without any DOCTYPE makes the parser request the
  external subset at `INSTANCE_START`, with `systemId = NULL`.

### Standalone and `keepProcessing`

The XML spec says that once a non-validating processor skips a parameter
entity reference (because it does not read it), it must not process later
declarations, since the skipped entity might have changed their meaning,
unless the document is standalone. Expat implements this with
`dtd->keepProcessing = dtd->standalone` whenever a PE reference is not
expanded (PE parsing off, no handler, undeclared PE, or the handler did not
read it). Almost every declaration action is guarded by `keepProcessing`.

`NotStandaloneHandler` is called when the document turns out to depend on
external markup (it has an external subset or PE reference) but does not say
`standalone="yes"`. If the handler returns 0, parsing fails with
`XML_ERROR_NOT_STANDALONE`. With `XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE`,
reading `standalone="yes"` turns PE parsing off.

### When is an undeclared entity an error?

The well-formedness constraint *Entity Declared* applies only when the
document cannot have hidden declarations. In content and attribute values:

```c
if (! dtd->hasParamEntityRefs || dtd->standalone) {
  if (! entity)               return XML_ERROR_UNDEFINED_ENTITY;
  if (! entity->is_internal)  return XML_ERROR_ENTITY_DECLARED_IN_PE;
} else if (! entity) {
  // not an error: report via SkippedEntityHandler (or default handler) and continue
}
```

In the prolog (attribute defaults, PE references) the condition is
`documentEntity && (standalone ? !openInternalEntities : !hasParamEntityRefs)`.

## 5.4 Expanding internal entities without recursion

Up to Expat 2.6 internal entities were expanded by recursive calls
(`doContent` → `processInternalEntity` → `doContent` …). Deeply nested
entities could overflow the C stack (CVE-2024-8176). The current design uses
**explicit stacks**. There are three kinds of expansion context, each with its
own stack and free list of `OPEN_INTERNAL_ENTITY` records:

| `enum EntityType` | Stack | Used for | Driver |
|-------------------|-------|----------|--------|
| `ENTITY_INTERNAL` | `m_openInternalEntities` | `&e;` in content, `%pe;` between declarations | `internalEntityProcessor` via `callProcessor`'s re-enter loop |
| `ENTITY_ATTRIBUTE` | `m_openAttributeEntities` | `&e;` inside an attribute value | the `while (1)` loop in `storeAttributeValue` |
| `ENTITY_VALUE` | `m_openValueEntities` | `%pe;` inside an entity value | the loop in `callStoreEntityValue` |

```c
typedef struct open_internal_entity {
  const char *internalEventPtr;     // current token inside the entity text
  const char *internalEventEndPtr;  //   (for errors and XML_DefaultCurrent)
  struct open_internal_entity *next;
  ENTITY *entity;
  int startTagLevel;                // m_tagLevel when expansion started
  XML_Bool betweenDecl;             // WFC: PE Between Declarations
  enum EntityType type;
} OPEN_INTERNAL_ENTITY;
```

**`processEntity(parser, entity, betweenDecl, type)`** (`xmlparse.c:6349`)
takes a record from the free list (or allocates one) and sets
`entity->open = hasMore = TRUE` and `processed = 0`. It pushes the record and
records `startTagLevel`. For `ENTITY_INTERNAL` it also sets
`m_processor = internalEntityProcessor` and calls `triggerReenter()`. It does
**not** parse anything.

**`internalEntityProcessor`** (`xmlparse.c:6410`) always works on the top of
the stack:

```
top = m_openInternalEntities
if top.entity.hasMore:
    text = entity.textPtr + entity.processed .. textPtr + textLen
    run doProlog (PE) or doContent (GE, startTagLevel = top.startTagLevel)
        on text with enc = m_internalEncoding, haveMore = FALSE
    if stopped early (suspended, or re-enter because a nested entity was pushed):
        entity.processed = offset reached; return
    entity.hasMore = FALSE
    if GE and m_tagLevel != startTagLevel: XML_ERROR_ASYNC_ENTITY
    triggerReenter(); return
else:                                  // finished, and all nested entities are done
    entity.open = FALSE; pop top onto free list
    if the stack is empty: m_processor = is_param ? prologProcessor : contentProcessor
    triggerReenter(); return
```

An entity is popped one iteration *after* its text is finished. Its `open`
flag therefore stays set while nested entities pushed during its text are
still being processed, which is what recursion detection needs.

When the stack empties, `m_processor` goes back to the document processor.
`callProcessor` loops once more and continues in the document buffer from
`*endPtr`, which `doContent` left just after the `&e;` that started the
expansion.

The attribute and entity-value loops follow the same pattern inline, because
they can't be suspended (attribute value handling has no callbacks), so
they don't need the processor/re-enter mechanism.

**Recursion detection** relies on `entity->open`. Seeing a reference to an
open entity gives `XML_ERROR_RECURSIVE_ENTITY_REF`. External entities are set
`open` around the handler call, and `getContext()`/`setContext()` pass the set
of open general entities on to child parsers.

**Amplification** is still possible without recursion (the "billion laughs"
attack, where each entity references the previous one ten times). It is
limited by the accounting described in
[07](07-memory-and-security.md#72-billion-laughs-protection-amplification-accounting).

## 5.5 External entities and child parsers

Expat **never does I/O**. For an external entity it calls

```c
int ExternalEntityRefHandler(XML_Parser parser /* or the arg set */,
                             const XML_Char *context,
                             const XML_Char *base, const XML_Char *systemId,
                             const XML_Char *publicId);
```

The application is expected to fetch the entity, create a child with
`XML_ExternalEntityParserCreate(parser, context, encoding)`, feed it with
`XML_Parse`, and free it. Returning 0 gives
`XML_ERROR_EXTERNAL_ENTITY_HANDLING` in the parent.

`XML_ExternalEntityParserCreate` (`xmlparse.c:1738`):

1. Saves all handlers and settings of the parent on the stack.
2. Calls `parserCreate(encodingName, &parent->m_mem, nsSep?, newDtd,
   parentParser)`:
   * `newDtd = parent DTD` for a PE (`context == NULL`); otherwise `NULL`, so
     a fresh DTD is created and filled by `dtdCopy`.
3. Copies handlers, `userData`, `handlerArg` (unless the parent used itself as
   the argument), the hash salt (**so copied hash tables stay valid**), the
   reparse-deferral flag, `m_ns_triplets`, PE-parsing mode, and
   `inEntityValue`.
4. Then:
   * **General entity** (`context != NULL`): `dtdCopy` and `setContext(context)`;
     the processor is `externalEntityInitProcessor`. The entity is parsed as
     *content* (`doContent` with `startTagLevel = 1`), with an optional
     leading text declaration. `m_tagLevel` starts at 1, so elements in the
     entity must balance.
   * **Parameter entity or external subset** (`context == NULL`): share the
     DTD, `m_isParamEntity = TRUE`, `XmlPrologStateInitExternalEntity`, and
     the processor is `externalParEntInitProcessor`. That processor sets
     `dtd->paramEntityRead = TRUE`, which is how the parent learns the entity
     was actually read.

A child parser:

* gets its root via `m_parentParser` (`getRootParserOf`). **All accounting
  and allocation limits are charged to the root parser**, so splitting a
  document into entities does not get around them.
* cannot be suspended while parsing a PE (`XML_ERROR_SUSPEND_PE`). A general
  entity parser *can* be suspended, but the application then has to manage
  resuming child and parent itself.
* must, for PEs, be created, used, and freed inside the handler call, because
  it shares and changes the parent's DTD.
