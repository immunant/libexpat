# 3. The prolog role machine: `xmlrole.c`

The prolog is everything before the root element: the XML declaration,
comments and PIs, and the `<!DOCTYPE …>` with its internal subset. External
DTD subsets and external parameter entities follow the same grammar. That
grammar is richer than the content grammar, so tokens alone are not enough.
`NAME` can be an element name, an attribute name, an attribute type keyword,
an entity name, `SYSTEM`, and so on. `xmlrole.c` resolves this.

## 3.1 Interface

```c
typedef struct prolog_state {            // xmlrole.h:112
  int (PTRCALL *handler)(struct prolog_state *state, int tok,
                         const char *ptr, const char *end,
                         const ENCODING *enc);
  unsigned level;          // parenthesis nesting depth (content models, enums)
  int role_none;           // role to return for "nothing interesting" in declClose
#ifdef XML_DTD
  unsigned includeLevel;   // nesting of <![INCLUDE[ ... ]]>
  int documentEntity;      // 1 = document entity, 0 = external subset / PE
  int inEntityValue;       // external PE being read as part of an entity value
#endif
} PROLOG_STATE;

#define XmlTokenRole(state, tok, ptr, end, enc) \
  (((state)->handler)(state, tok, ptr, end, enc))
```

The parser calls `XmlTokenRole(&parser->m_prologState, tok, s, next, enc)` for
every prolog token and gets back an `XML_ROLE_*` value (`xmlrole.h`). Each
handler returns a role and, as a side effect, sets `state->handler` to the
next state. **The current state is the function pointer.** There is no state
enum and no transition table.

`XmlPrologStateInit` starts in `prolog0`. `XmlPrologStateInitExternalEntity`
(for external subsets and PEs) starts in `externalSubset0` with
`documentEntity = 0`.

## 3.2 State graph

```
prolog0 ──XML_DECL/PI/COMMENT/S──► prolog1 ──<!DOCTYPE──► doctype0
   │                                  │                       │ NAME
   │<!DOCTYPE                         │INSTANCE_START          ▼
   └──────────────────────────────────┼──────────────────► doctype1
                                      ▼                  SYSTEM│PUBLIC│[ │>
                                    error                     ▼
                     (parser switches to contentProcessor) doctype2/3/4
                                                               │ [
                                                               ▼
     prolog2 ◄──>── doctype5 ◄──]── internalSubset ◄──────────┘
       │                               │ <!ENTITY   → entity0..10  ─┐
       │INSTANCE_START                 │ <!ATTLIST  → attlist0..9  ─┤
       ▼                               │ <!ELEMENT  → element0..7  ─┼─► declClose
     error                             │ <!NOTATION → notation0..4 ─┘     │ >
                                       │ %pe;  → PARAM_ENTITY_REF          ▼
                                       │ PI / COMMENT          setTopLevel(): back to
                                       │                       internalSubset or
                                       │                       externalSubset1

 External subset / PE (XML_DTD):
   externalSubset0 ─(TEXT_DECL?)─► externalSubset1 ─<![──► condSect0
                                       ▲  │ default: delegate   │ INCLUDE   │ IGNORE
                                       │  ▼ to internalSubset   ▼           ▼
                                       │                    condSect1   condSect2
                                       └──────────[ (includeLevel++)─┘   [ → IGNORE_SECT
```

Notes:

* Handlers with numbered suffixes follow one declaration's grammar step by
  step. `entity0` sees `%` or a name. `entity2` decides between `SYSTEM`,
  `PUBLIC`, or a literal value. `entity5` handles an optional `NDATA`, and so
  on.
* `declClose` is shared by all declarations. It waits for `>` and in the
  meantime returns `state->role_none`, which the previous handler set to the
  right `*_NONE` role. This lets the parser give whitespace inside a
  declaration to the default handler with the right context.
* `setTopLevel` returns to `internalSubset` for the document entity, or
  `externalSubset1` for external subsets and PEs.
* `level` tracks `(` … `)` nesting in `<!ELEMENT>` content models and
  `<!ATTLIST>` enumerations. `element7` goes to `declClose` only when
  `level` returns to 0.
* `common()` is the fallback for every handler. With `XML_DTD`, a
  `%pe;` inside a declaration in an external subset or PE becomes
  `XML_ROLE_INNER_PARAM_ENTITY_REF`; that is legal there but not in the
  internal subset. Anything else switches to the `error` sink and returns
  `XML_ROLE_ERROR`.
* `error()` always returns `XML_ROLE_NONE`. It is also the state after
  `INSTANCE_START`. It should never be reached because the parser changes
  processor first, and it is excluded from coverage.

## 3.3 What the role machine checks and what it doesn't

The header comment of `xmlrole.c` lists what is *not* checked:

* that `,` and `|` are not mixed in one model group, and
* the content of literals.

The parser does both: `m_groupConnector[level]` remembers whether a group is a
sequence or a choice and rejects a mix with `XML_ERROR_SYNTAX`. Literals are
decoded by `storeAttributeValue` and `storeEntityValue`.

Keywords are recognized with `XmlNameMatchesAscii(enc, ptr, end, KW_…)`.
`KW_*` constants are `char` arrays of `ASCII_*` codes, so matching works the
same way in UTF-16 and other encodings.

## 3.4 How the parser uses roles

`doProlog` in `xmlparse.c` is one large `switch (role)` (around 1000 lines).
The pattern for each case is:

1. Do the semantic work: intern a name, create an entity, store a literal,
   build a scaffold node, call a declaration handler.
2. If a specific handler consumed the token, set `handleDefault = XML_FALSE`.
3. After the switch, if `handleDefault` is still true and a default handler is
   set, pass the raw token text to it (`reportDefault`). This is how
   `XML_SetDefaultHandler` gets a faithful copy of every byte that no
   specific handler took.

Several roles carry information that is used later. For example
`XML_ROLE_ATTLIST_ELEMENT_NAME` sets `m_declElementType`,
`XML_ROLE_ATTRIBUTE_NAME` sets `m_declAttributeId`, and the
`XML_ROLE_ATTRIBUTE_TYPE_*` roles set `m_declAttributeType`. When
`XML_ROLE_DEFAULT_ATTRIBUTE_VALUE` arrives, the parser has everything it
needs to call `defineAttribute` and the `AttlistDecl` handler. The `m_decl*`
fields of the parser struct act as the role machine's "registers".

See [05-dtd-and-entities.md](05-dtd-and-entities.md) for what each group of
roles does to the DTD.

## 3.5 Appendix: complete transition table

`S` = `XML_TOK_PROLOG_S`. Unless listed otherwise, `S` returns the state's
"none" role and stays in the same state. Every token not listed goes to
`common()`, giving `XML_ROLE_ERROR` (or `INNER_PARAM_ENTITY_REF` for `%pe;`
outside the document entity). `KW` means the token's text must equal that
keyword exactly (case-sensitive, ASCII). `→ top` means `setTopLevel()`:
`internalSubset` in the document entity, `externalSubset1` otherwise.

| State | Token (condition) | Role returned | Next state |
|-------|-------------------|---------------|------------|
| **prolog0** | `S` | NONE | prolog1 |
| | `XML_DECL` | XML_DECL | prolog1 |
| | `PI` / `COMMENT` | PI / COMMENT | prolog1 |
| | `BOM` | NONE | prolog0 |
| | `DECL_OPEN` (KW DOCTYPE) | DOCTYPE_NONE | doctype0 |
| | `INSTANCE_START` | INSTANCE_START | error |
| **prolog1** | `S`, `PI`, `COMMENT` | NONE / PI / COMMENT | prolog1 |
| | `DECL_OPEN` (KW DOCTYPE) | DOCTYPE_NONE | doctype0 |
| | `INSTANCE_START` | INSTANCE_START | error |
| **prolog2** (after DOCTYPE) | `S`, `PI`, `COMMENT` | NONE / PI / COMMENT | prolog2 |
| | `INSTANCE_START` | INSTANCE_START | error |
| **doctype0** | `NAME`, `PREFIXED_NAME` | DOCTYPE_NAME | doctype1 |
| **doctype1** | `[` | DOCTYPE_INTERNAL_SUBSET | internalSubset |
| | `>` | DOCTYPE_CLOSE | prolog2 |
| | `NAME` KW SYSTEM / PUBLIC | DOCTYPE_NONE | doctype3 / doctype2 |
| **doctype2** | `LITERAL` | DOCTYPE_PUBLIC_ID | doctype3 |
| **doctype3** | `LITERAL` | DOCTYPE_SYSTEM_ID | doctype4 |
| **doctype4** | `[` / `>` | DOCTYPE_INTERNAL_SUBSET / DOCTYPE_CLOSE | internalSubset / prolog2 |
| **doctype5** (after `]`) | `>` | DOCTYPE_CLOSE | prolog2 |
| **internalSubset** | `DECL_OPEN` KW ENTITY / ATTLIST / ELEMENT / NOTATION | ENTITY_NONE / ATTLIST_NONE / ELEMENT_NONE / NOTATION_NONE | entity0 / attlist0 / element0 / notation0 |
| | `PI` / `COMMENT` | PI / COMMENT | — |
| | `PARAM_ENTITY_REF` | PARAM_ENTITY_REF | — |
| | `]` | DOCTYPE_NONE | doctype5 |
| | `NONE` (end of input) | NONE | — |
| **externalSubset0** | `XML_DECL` | TEXT_DECL | externalSubset1 |
| | anything else | delegated to externalSubset1 | |
| **externalSubset1** | `<![` | NONE | condSect0 |
| | `]]>` (includeLevel > 0) | NONE (includeLevel--) | — |
| | `NONE` (includeLevel == 0) | NONE | — |
| | `]`, or `NONE` with includeLevel > 0 | → `common` (error) | |
| | anything else | delegated to internalSubset | |
| **condSect0** | `NAME` KW INCLUDE / IGNORE | NONE | condSect1 / condSect2 |
| **condSect1** | `[` | NONE (includeLevel++) | externalSubset1 |
| **condSect2** | `[` | IGNORE_SECT | externalSubset1 (the parser skips the section with `XmlIgnoreSectionTok`) |
| **entity0** | `%` | ENTITY_NONE | entity1 |
| | `NAME` | GENERAL_ENTITY_NAME | entity2 |
| **entity1** | `NAME` | PARAM_ENTITY_NAME | entity7 |
| **entity2** | `NAME` KW SYSTEM / PUBLIC | ENTITY_NONE | entity4 / entity3 |
| | `LITERAL` | ENTITY_VALUE | declClose(ENTITY_NONE) |
| **entity3** | `LITERAL` | ENTITY_PUBLIC_ID | entity4 |
| **entity4** | `LITERAL` | ENTITY_SYSTEM_ID | entity5 |
| **entity5** | `>` | ENTITY_COMPLETE | → top |
| | `NAME` KW NDATA | ENTITY_NONE | entity6 |
| **entity6** | `NAME` | ENTITY_NOTATION_NAME | declClose(ENTITY_NONE) |
| **entity7** (PE) | `NAME` KW SYSTEM / PUBLIC | ENTITY_NONE | entity9 / entity8 |
| | `LITERAL` | ENTITY_VALUE | declClose(ENTITY_NONE) |
| **entity8** | `LITERAL` | ENTITY_PUBLIC_ID | entity9 |
| **entity9** | `LITERAL` | ENTITY_SYSTEM_ID | entity10 |
| **entity10** | `>` | ENTITY_COMPLETE | → top (no NDATA for PEs) |
| **notation0** | `NAME` | NOTATION_NAME | notation1 |
| **notation1** | `NAME` KW SYSTEM / PUBLIC | NOTATION_NONE | notation3 / notation2 |
| **notation2** | `LITERAL` | NOTATION_PUBLIC_ID | notation4 |
| **notation3** | `LITERAL` | NOTATION_SYSTEM_ID | declClose(NOTATION_NONE) |
| **notation4** | `LITERAL` | NOTATION_SYSTEM_ID | declClose(NOTATION_NONE) |
| | `>` | NOTATION_NO_SYSTEM_ID | → top |
| **attlist0** | `NAME`, `PREFIXED_NAME` | ATTLIST_ELEMENT_NAME | attlist1 |
| **attlist1** | `>` | ATTLIST_NONE | → top |
| | `NAME`, `PREFIXED_NAME` | ATTRIBUTE_NAME | attlist2 |
| **attlist2** | `NAME` KW CDATA, ID, IDREF, IDREFS, ENTITY, ENTITIES, NMTOKEN, NMTOKENS | ATTRIBUTE_TYPE_CDATA + i (same order) | attlist8 |
| | `NAME` KW NOTATION | ATTLIST_NONE | attlist5 |
| | `(` | ATTLIST_NONE | attlist3 |
| **attlist3** | `NMTOKEN`, `NAME`, `PREFIXED_NAME` | ATTRIBUTE_ENUM_VALUE | attlist4 |
| **attlist4** | `)` / `\|` | ATTLIST_NONE | attlist8 / attlist3 |
| **attlist5** | `(` | ATTLIST_NONE | attlist6 |
| **attlist6** | `NAME` (not prefixed) | ATTRIBUTE_NOTATION_VALUE | attlist7 |
| **attlist7** | `)` / `\|` | ATTLIST_NONE | attlist8 / attlist6 |
| **attlist8** | `POUND_NAME` #IMPLIED / #REQUIRED | IMPLIED_ / REQUIRED_ATTRIBUTE_VALUE | attlist1 |
| | `POUND_NAME` #FIXED | ATTLIST_NONE | attlist9 |
| | `LITERAL` | DEFAULT_ATTRIBUTE_VALUE | attlist1 |
| **attlist9** | `LITERAL` | FIXED_ATTRIBUTE_VALUE | attlist1 |
| **element0** | `NAME`, `PREFIXED_NAME` | ELEMENT_NAME | element1 |
| **element1** | `NAME` KW EMPTY / ANY | CONTENT_EMPTY / CONTENT_ANY | declClose(ELEMENT_NONE) |
| | `(` | GROUP_OPEN (level = 1) | element2 |
| **element2** | `POUND_NAME` #PCDATA | CONTENT_PCDATA | element3 |
| | `(` | GROUP_OPEN (level = 2) | element6 |
| | `NAME`/`PREFIXED_NAME`, `NAME?`, `NAME*`, `NAME+` | CONTENT_ELEMENT(_OPT/_REP/_PLUS) | element7 |
| **element3** (mixed) | `)` / `)*` | GROUP_CLOSE / GROUP_CLOSE_REP | declClose(ELEMENT_NONE) |
| | `\|` | ELEMENT_NONE | element4 |
| **element4** | `NAME`, `PREFIXED_NAME` | CONTENT_ELEMENT | element5 |
| **element5** | `)*` (only) | GROUP_CLOSE_REP | declClose(ELEMENT_NONE) |
| | `\|` | ELEMENT_NONE | element4 |
| **element6** | `(` | GROUP_OPEN (level++) | element6 |
| | names (with suffixes) | CONTENT_ELEMENT(...) | element7 |
| **element7** | `)`, `)*`, `)?`, `)+` | GROUP_CLOSE(_REP/_OPT/_PLUS) (level--) | element7, or declClose(ELEMENT_NONE) when level hits 0 |
| | `,` / `\|` | GROUP_SEQUENCE / GROUP_CHOICE | element6 |
| **declClose** | `S` | `role_none` | — |
| | `>` | `role_none` | → top |
| **error** | anything | NONE | error |

Notes:

* In mixed content (`element3..5`), `|` gives `ELEMENT_NONE` rather than
  `GROUP_CHOICE`. The scaffold node stays `XML_CTYPE_MIXED` with `NAME`
  children. Mixed content with element names must close with `)*`. `(#PCDATA)`
  may close with `)` or `)*`, and that sets the quantifier to `NONE` or
  `REP`.
* In namespace builds, prefixed names are accepted for DOCTYPE, element, and
  attribute names, enumeration values, and content particles, but **not** for
  entity or notation names, or notation-type values.
* The `XML_ROLE_*` enum order matters for `ATTRIBUTE_TYPE_CDATA + i`.
