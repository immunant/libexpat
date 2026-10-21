# DTD and entity processing

DTD processing has two layers: [`xmlrole.c`](../expat/lib/xmlrole.c) recognizes
grammar, while `doProlog` and its helpers in
[`xmlparse.c`](../expat/lib/xmlparse.c) apply semantic effects. Turning off
`XML_DTD` removes parameter-entity/external-subset machinery, not all recognition
of an internal DOCTYPE or all declaration-driven behavior. `XML_GE` independently
controls general-entity support; `XML_DTD` requires `XML_GE == 1`.

## Role grammar

`PROLOG_STATE.handler` points to the current grammar function. A function consumes
one lexical token, chooses its next function, and returns a role. A quoted
literal can thus become an entity value, system identifier, public identifier,
or default attribute value according to state. Lexical shape alone is insufficient.

| State family in xmlrole.c | Grammar responsibility |
| --- | --- |
| `prolog0`, `prolog1`, `prolog2` | Initial XML declaration opportunity, miscellaneous markup, at most one DOCTYPE, then instance start |
| `doctype0` through `doctype5` | Root name, optional external identifier, optional internal subset, close |
| `internalSubset` | Declaration/comment/PI/parameter-reference dispatch inside document DTD |
| `entity0` through `entity10` | General/parameter declaration names, internal values, external IDs, optional NDATA notation |
| `notation0` through `notation4` | Notation names and permitted external identifier forms |
| `attlist0` through `attlist9` | Element name, repeated attribute declarations, type/enumeration/notation, default policy |
| `element0` through `element7` | EMPTY, ANY, mixed content, sequence/choice groups, quantifiers |
| `externalSubset0`, `externalSubset1` | External text-declaration opportunity and external DTD declarations |
| `condSect0` through `condSect2` | INCLUDE/IGNORE conditional sections where permitted |
| `declClose` | Shared close/whitespace path after a declaration |
| `common`, `error` | Inner parameter-reference exception in external grammar; invalid transition handling |

`level` tracks model-group nesting. `includeLevel` balances included conditional
sections. `documentEntity` distinguishes internal-subset restrictions from
external grammar. `inEntityValue` communicates the special replacement-value
context to external parameter parsing.

The role machine deliberately does not check literal contents or mixing `,` and
`|` in one model group. Semantic code validates public IDs, normalizes/expands
values, and uses `m_groupConnector[level]` to reject inconsistent connectors.
A reimplementation can combine the layers, but must carry both kinds of checks.

## Declaration actions

`doProlog` holds the current declaration's partially assembled fields in the
parser and dispatches roles as follows:

| Role family | Semantic result |
| --- | --- |
| DOCTYPE name/IDs/subset | Save callback fields, mark possible external declarations, emit start before subset contents and end after external-subset handling |
| General/parameter entity name | Intern `ENTITY`, with first declaration taking precedence; skip replacing predefined entities |
| Internal entity value | Store processed replacement text in `entityValuePool`; emit length-delimited declaration callback |
| Entity system/public IDs | Persist identifiers and current base; public-ID whitespace is normalized |
| Entity notation | Mark unparsed entity; old unparsed-declaration handler takes precedence over general entity-declaration handler for that event |
| ATTLIST | Intern element/attribute; record normalization, optional ID metadata, optional normalized default; emit callback once per attribute |
| NOTATION | Validate/normalize public ID and emit notation callback with available IDs |
| ELEMENT and group roles | Validate syntax; build optional content-model scaffold and deliver a model if a handler is installed |
| Parameter reference | Expand/read/skip according to context, declarations, policy, standalone status, and feature build |
| Instance start | Switch to content processor without consuming the first element token |

`XML_SetBase` copies a base string. External entity declarations capture the base
at declaration time; loading and resolution are application responsibilities.
System identifiers are not rewritten into absolute URLs by Expat.

Declarations remain meaningful without declaration callbacks. For example,
ATTLIST information changes later attribute normalization and inserts defaults.
An ELEMENT model is a reporting structure, however, not a validator that checks
the later child sequence. `#REQUIRED` does not enforce presence; `#FIXED` does
not generally enforce equality; ID/IDREF declarations do not establish global
uniqueness/reference validation.

## DTD completeness and standalone state

`DTD.keepProcessing` records whether subsequent declarations can still be used
after skipping parameter-entity input. `hasParamEntityRefs` records encountered
PE references, including an external subset; `standalone` records the document's
`standalone='yes'`. `paramEntityRead` tells a caller whether an external PE parser
actually supplied declarations.

These flags affect undefined entities and declaration-origin checks. In content,
when no parameter references could hide declarations, or the document is
standalone, an undeclared general entity is an error. If external/PE declarations
may be absent, a missing declaration can instead be skipped. An entity declared
outside the qualifying internal context can trigger
`XML_ERROR_ENTITY_DECLARED_IN_PE`. The attribute/default-value paths have related
but context-specific checks; preserve their predicates rather than using a
single global 'unknown entity is always fatal' rule.

`ENTITY.is_internal` means declared in the internal subset outside a parameter
entity, not merely 'has internal replacement text'. The latter is indicated by
`textPtr != NULL`. These are independent properties.

`XML_SetParamEntityParsing` offers NEVER (default), UNLESS_STANDALONE, and ALWAYS.
The standalone declaration disables UNLESS_STANDALONE processing as appropriate.
`XML_UseForeignDTD(true)` allows an application-supplied external subset even
without a document DOCTYPE. Its external callback has null context/system/public
IDs; no synthetic DOCTYPE callbacks are created for an absent DOCTYPE.

The not-standalone handler may reject documents by returning zero; successful
external PE reading and policy influence when it runs. This is a particular
callback/error contract, not full DTD validation.

## Replacement text is processed in stages

During an entity declaration, `storeEntityValue`:

1. Converts source bytes into the internal/API representation.
2. Normalizes source line endings to LF.
3. Resolves numeric references to characters.
4. Preserves general references as reference spelling for later interpretation.
5. Expands parameter references only where the DTD context allows them.

The stored text is length-delimited and can be empty while still having a
nonnull pointer. No NUL terminator is promised by the entity declaration
callback. A null value instead signals an external declaration.

At a subsequent use, the text is tokenized again in the usage context: content,
attribute normalization, or another declaration's replacement value. This is
essential to nested entity semantics. Do not eagerly replace every named
reference when its declaration is first encountered.

For example:

```xml
<!DOCTYPE r [
  <!ENTITY word "world">
  <!ENTITY greeting "hello &word;">
]>
<r>&greeting;</r>
```

The stored `greeting` still includes `&word;`. Expanding it schedules a `word`
frame; the logical content is `hello world`. Reference spelling that produces
markup through declaration-time numeric replacement can affect later tokenization,
whereas `&lt;` handled directly as a predefined reference emits a character.
Replacement timing must therefore be context-aware.

## Iterative internal entity work

`processEntity` gets an `OPEN_INTERNAL_ENTITY` from a free list or allocates one.
It sets `open`, `hasMore`, `processed=0`, captures starting tag depth and grammar
context, and pushes onto the category-specific list.

For content/prolog entities it selects `internalEntityProcessor` and sets
`m_reenter`. `callProcessor` iterates the following work:

```text
top = open internal entity
if top.entity.hasMore:
    parse replacement from processed offset in internal encoding
    if nested entity or suspension interrupts:
        save byte offset and return to dispatcher
    mark own text finished
    check general-entity element-depth boundary
    request another dispatcher iteration
else:
    pop frame, clear open, recycle frame
    if no entity remains: restore content or prolog processor
    request another dispatcher iteration
```

The parent stays open until nested work finishes, even if the nested reference
was its last token. This prevents delayed interpretation from evading recursion
detection. Separate iterative loops in `storeAttributeValue` and
`callStoreEntityValue` manage attribute and declaration-value entity lists.

A general entity's replacement text must be balanced at its entry tag depth:
it cannot close an element opened outside it or leave an element for its caller
to close. `startTagLevel` and `XML_ERROR_ASYNC_ENTITY` implement this boundary.
Parameter entities have grammar-boundary rules instead; an expanded PE must not
close the document DOCTYPE (`allowClosingDoctype`), and references between
declarations must supply complete permitted markup.

## Context-specific restrictions

| Context | General entities | Parameter entities |
| --- | --- | --- |
| Element content | Internal text parsed as content; external general handler if installed; unparsed entity use is an error | `%name;` is ordinary content text |
| Attribute value | Internal text expanded and normalized; external parsed entities and unparsed entities are errors | No PE substitution |
| Entity declaration value | General references preserved, numeric references resolved | Expand where allowed in external/PE contexts; forbidden inside internal-subset declarations |
| DTD between declarations | Not ordinary general-reference content | PE policy, completeness, recursion, and standalone rules apply |

`XML_SetDefaultHandler` suppresses normal internal general-entity expansion in
content. References go to a skipped-entity handler if installed, otherwise to
default reporting. `XML_SetDefaultHandlerExpand` keeps expansion enabled.
This setting does not eliminate required attribute normalization.

Skipped entities cannot always be reported: events for unknown entities inside
attribute values or skipped PEs within declarations would be out of sync with
the enclosing callback. Missing an external general handler is also a distinct
path from a missing declaration. Follow `doContent`, `appendAttributeValue`, and
`doProlog` individually.

With `XML_GE == 0`, declared general entities get synthetic replacement text such
as `&amp;name;`, so their use becomes literal `&name;` rather than ordinary
replacement. This is not equivalent to deleting all entity-related branches;
predefined and numeric references still have their own behavior.

## External parser relationship

The library never fetches a file or URL itself. An external-entity callback
receives IDs, base, and a context. Returning zero makes the referring parser
fail with `XML_ERROR_EXTERNAL_ENTITY_HANDLING`. To load an entity, the application
creates a child through `XML_ExternalEntityParserCreate`, feeds its bytes, and
frees it at the required point.

| Child kind | Context | State relationship | Parsing contract |
| --- | --- | --- | --- |
| External general entity | Nonnull, possibly empty string | Copy declaration tables/defaults/text; restore namespace and open-general-entity context; inherit most handlers and options | Content fragment, possibly multiple top-level pieces or empty; optional text declaration; balanced tags |
| External parameter entity/subset | Null | Share parent's DTD; initialize external prolog grammar; use value processor when supplying entity replacement text | Synchronous create/parse/free within external callback; no calls on old parser until child free; no resumable suspension |

Context is a NUL-terminated `XML_Char` string with form-feed-separated tokens:
`name` marks an open general entity, `prefix=uri` supplies a namespace binding,
and `=uri` supplies a default namespace. `getContext`/`setContext` serialize and
restore this state. Context expires when the callback returns; deferred general
entity parsing must copy it.

Children inherit allocation suite, many handler registrations, user data,
namespace/triplet settings, PE policy, salt, and reparse-deferral setting.
Inheritance is explicit in `XML_ExternalEntityParserCreate`; notably start/end
DOCTYPE handlers are not copied there. Encoding and base choices should be
checked separately rather than assuming a complete parser-state clone.
All descendants contribute to root resource budgets and use the root hash secret.

## Content model representation

With an element-declaration handler, the parser builds an index-based
`CONTENT_SCAFFOLD` graph, recording child/sibling relationships, type, name,
and `XML_CQUANT_*` quantifier. `scaffIndex` tracks open groups by level.

`build_model` allocates one raw-suite block: `XML_Content` nodes followed by all
name strings. It fills nodes iteratively, using temporary source indices in
destination cells as a work queue. Siblings are adjacent so `children[i]` works.
No native recursion is needed even for a deeply nested model. EMPTY and ANY
use simpler one-node allocations.

The model's public types are EMPTY, ANY, MIXED, NAME, CHOICE, and SEQ; quantifiers
are NONE, OPT, REP, and PLUS. The application owns the result and eventually
calls `XML_FreeContentModel`. The parser does not retain it as a document-content
validation automaton.
