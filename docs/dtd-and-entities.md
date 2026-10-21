# Expat DTD and entity processing

[Guide index](README.md)

DTD declarations affect subsequent parsing even though Expat is non-validating. The grammar is in [`xmlrole.c`](../expat/lib/xmlrole.c); semantic effects are in `doProlog` and the entity helpers in [`xmlparse.c`](../expat/lib/xmlparse.c).

## Grammar roles

`PROLOG_STATE.handler` is a function pointer representing the current grammar state. `XmlTokenRole` calls it with the next lexical token and its source range. The state handler checks permitted syntax, selects the next handler, and returns an `XML_ROLE_*` value. It does not allocate entities or invoke application declaration callbacks.

For example, the token `XML_TOK_NAME` can yield `XML_ROLE_DOCTYPE_NAME`, `XML_ROLE_GENERAL_ENTITY_NAME`, `XML_ROLE_ATTRIBUTE_NAME`, or `XML_ROLE_CONTENT_ELEMENT`. The lexical class alone does not determine what to store.

| State family | Grammar covered |
| --- | --- |
| `prolog0`, `prolog1`, `prolog2` | Initial declaration position, miscellaneous prolog content, optional single DOCTYPE, and transition to the document element |
| `doctype0` through `doctype5` | DOCTYPE name, external identifiers, internal-subset opening and closing |
| `internalSubset` | Dispatch to entity, element, attribute-list, and notation declarations; inter-declaration comments, PIs, and parameter references |
| `externalSubset0`, `externalSubset1` | Optional text declaration and external-subset rules, including conditional sections |
| `entity0` through `entity10` | General/parameter declarations, internal values, external identifiers, and NDATA notation |
| `notation0` through `notation4` | Public/system identifiers for notation declarations |
| `attlist0` through `attlist9` | Element name, attribute names and types, enumerations, defaults, `#IMPLIED`, `#REQUIRED`, and `#FIXED` |
| `element0` through `element7` | `EMPTY`, `ANY`, mixed content, nested choice/sequence groups, and quantifiers |
| `condSect0` through `condSect2` | External DTD `INCLUDE` and `IGNORE` section syntax |
| `declClose`, `common`, `error` | Shared closing transitions and error handling |

`level` tracks nested content-model groups; `includeLevel` tracks included conditional sections. `documentEntity` distinguishes the document/internal-subset grammar from external DTD contexts. `inEntityValue` selects special handling when an external parameter entity contributes to a declaration value.

An implementation need not reproduce these function names, but it must retain the context distinctions. In particular, parameter references inside markup declarations are disallowed in the internal subset; the external-subset grammar has additional permitted positions.

## Declaration effects

`doProlog` uses role results to update current-declaration fields and DTD tables. It also decides whether a token belongs to a specialized callback or should reach the default handler. Specialized declaration handlers suppress their corresponding default reporting; default reporting is not an independent copy of every input token.

| Declaration | Stored or reported result |
| --- | --- |
| XML declaration | Version/encoding/standalone callback; encoding selection; standalone policy |
| DOCTYPE | Start/end callbacks; external subset identifiers; optional external-handler invocation |
| ENTITY | Name, replacement text or IDs, declaration base, parameter/general distinction, provenance, optional notation |
| ATTLIST | Element-specific type/default metadata, normalized default values, ID identity; one declaration callback per attribute |
| ELEMENT | Content-model scaffold and optional application-owned `XML_Content` tree |
| NOTATION | Declaration callback with normalized public ID and optional system ID |

The first retained entity declaration wins: if `lookup` finds an existing object, the new declaration does not overwrite it. Predefined entities have their own recognition path. Attribute declarations also retain the relevant first declaration metadata through `defineAttribute`.

`#REQUIRED` and `#IMPLIED` carry no default value. `#FIXED` supplies a default but does not make this non-validating parser enforce equality for an explicitly supplied value. Expat likewise does not validate required-attribute presence, ID uniqueness or IDREF resolution, enumerated-value membership, or element children against the declared model. It still checks declaration syntax and the well-formedness constraints used by its parser.

## Content-model construction

`CONTENT_SCAFFOLD` is an index-based temporary tree. `nextScaffoldPart`, `scaffIndex`, and the grammar's group state accumulate sequence/choice/mixed/name nodes and quantifiers. `build_model` converts that scaffold iteratively into one allocation containing `XML_Content` records and trailing name strings.

Sibling children occupy contiguous records, satisfying the public `children` array contract. The construction loop uses pending work recorded in output slots instead of recursive C calls, so deeply nested declarations do not require corresponding native-stack depth. The declaration callback takes ownership of the resulting model. Models describe declarations; they are not document nodes or a validation automaton.

## The three entity-processing contexts

| Context | Entry points | Semantic differences |
| --- | --- | --- |
| Element content or DTD parameter expansion | `processEntity`, `internalEntityProcessor`, `doContent`/`doProlog` | Can produce element/text/declaration events; retains continuation across suspension |
| Attribute value | `storeAttributeValue`, `appendAttributeValue` | Expands internal general references and normalizes whitespace; rejects external general entities and literal invalid markup |
| Entity declaration value | `callStoreEntityValue`, `storeEntityValue` | Resolves character references and permitted parameter references while retaining general references for later expansion |

Each context has its own open/free frame lists. Reusing one recursive string-substitution routine for all three loses normalization, provenance, and suspension distinctions.

For example:

```xml
<!DOCTYPE r [
  <!ENTITY a "A">
  <!ENTITY b "&a;B">
]>
<r>&b;</r>
```

The stored value of `b` includes the general reference `&a;`. On use, `b` is opened, its reference opens `a`, `A` is reported, then the remaining `B` is reported. `b` remains open while its child expansion is pending. If `a` instead refers back to `b`, `ENTITY.open` detects the cycle.

Replacement text can contain markup, not just characters. Consequently an expansion must be parsed in its surrounding grammar, and a general entity's elements must be balanced within that entity. `startTagLevel` checks reject attempts to open an element in one entity and close it in another (`XML_ERROR_ASYNC_ENTITY`).

`ENTITY.textPtr` is not necessarily null-terminated; `textLen` controls the range. The stored text is in internal/API encoding. The resume offset `processed` is measured in bytes, including for UTF-16 builds.

## Entity declaration values and normalization

When storing an internal entity value, Expat converts it, normalizes literal line endings, resolves numeric references, and expands permitted parameter references. General references are copied for later interpretation. An empty internal value still has a non-null pointer, because a null value in an entity-declaration callback signifies an external entity.

Attribute values instead expand internal general entities immediately, apply whitespace normalization appropriate to the declared type, and reject references to external or unparsed entities. Numeric references preserve their referenced characters; literal whitespace follows attribute rules. An entity-expanded quote is data inside the already delimited attribute value, not a new closing delimiter.

The five predefined references (`amp`, `lt`, `gt`, `apos`, `quot`) are recognized directly. General entity lookup adds checks for undefined names, recursion, unparsed entities, and declaration provenance. Whether a missing declaration is fatal depends on standalone state and whether potentially relevant parameter/external declarations were skipped.

## Skipped declarations and standalone state

Three DTD flags are particularly important:

- `hasParamEntityRefs` records that parameter references or an external subset could affect the available declarations.
- `keepProcessing` becomes false when skipped parameter entities mean later declarations cannot safely be retained under the applicable non-validating rules.
- `standalone` records `standalone="yes"` and tightens entity-declaration checks.

`ENTITY.is_internal` distinguishes declarations in the internal subset outside parameter entities from declarations reached through a parent parser or parameter expansion. Entity use consults this provenance. Do not flatten all declarations into an undifferentiated name-to-string dictionary.

`XML_SetSkippedEntityHandler` reports permitted skipped references, including internal references suppressed by `XML_SetDefaultHandler`. It cannot report skipped general entities inside attributes or parameter entities inside declarations as independent events without disrupting their enclosing event, and the implementation deliberately omits those notifications.

`XML_NotStandaloneHandler` can veto processing when the relevant non-standalone condition is encountered; a false return produces `XML_ERROR_NOT_STANDALONE`. Parameter-entity parsing policy changes when this notification occurs.

## External entity protocol

Expat does not open the system identifier itself. The application receives an external-reference callback, resolves/loads the resource under its own policy, creates a child parser, supplies bytes, and frees the child. Handler failure causes the calling parser to report `XML_ERROR_EXTERNAL_ENTITY_HANDLING`; the child may have a more specific parse error for the application to inspect.

| Property | External general entity | External parameter entity or external subset |
| --- | --- | --- |
| Callback context | Non-null serialized context, possibly an empty string | Null |
| DTD state | Newly allocated DTD populated by `dtdCopy` | Shared parent DTD |
| Namespace/open-entity context | Restored through `setContext` | Uses shared state; must not overwrite prefix bindings with child-owned bindings |
| Grammar | Optional text declaration followed by a balanced content fragment | External DTD grammar or declaration-value processing |
| Timing | Can be handled during callback or deferred with copied context and a live parent | Must complete creation, parsing, and freeing synchronously inside callback |
| Suspension | General-entity parser can suspend | Resumable stopping is prohibited |

Child parsers inherit allocator, ordinary handlers and user data, namespace settings, parameter-entity policy, and reparse-deferral setting according to `XML_ExternalEntityParserCreate`. They retain `m_parentParser` even when their DTD is copied. Hash and resource accounting therefore still reach the root. Keep the parser family alive and serialize access to shared accounting; a copied DTD alone does not establish independent resource-state ownership.

The public header describes general-entity parsers as independent, but this revision's implementation shares root resource state through the parent pointer. A port should explicitly resolve that distinction in its lifetime/concurrency contract instead of assuming the DTD copy permits freeing the parent or unsynchronized family access.

`getContext` serializes formfeed-separated tokens: an entity name marks an already-open general entity; `prefix=uri` supplies a prefix binding; `=uri` supplies the default namespace. `setContext` restores these facts so a child detects recursive references and sees the namespace environment at the reference site. Token order follows table iteration and is not a semantic ordering guarantee.

`XML_SetBase` supplies stored base metadata. It does not resolve URIs or automatically adopt the system identifier of a newly loaded resource; the application can set a child's base appropriately.

## Parameter entity policy and foreign DTDs

`XML_SetParamEntityParsing` selects `NEVER`, `UNLESS_STANDALONE`, or `ALWAYS`, before parsing starts. With `UNLESS_STANDALONE`, a standalone declaration disables parameter parsing. Without compiled DTD support, requesting enabled parameter parsing fails.

`XML_UseForeignDTD` lets an application supply an external subset even when the input does not specify one. The external-reference callback can then receive null identifiers as well as null context. `doProlog` invokes it at DOCTYPE completion or just before document content when there is no DOCTYPE. `paramEntityRead` distinguishes actually parsed external declarations from a callback that returned success without supplying any.

Conditional sections belong to external DTD contexts. INCLUDE continues declaration parsing with balanced include depth; IGNORE scans nested delimiters without interpreting enclosed declarations. Parameter entity boundaries must also satisfy declaration-completeness checks (`XML_ERROR_INCOMPLETE_PE`), and expanded parameter text cannot close the surrounding document DOCTYPE.
