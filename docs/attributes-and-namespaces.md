# Attributes and namespaces

The central routine is `storeAtts` in
[`xmlparse.c`](../expat/lib/xmlparse.c), with `getAttributeId`,
`setElementTypePrefix`, `defineAttribute`, `storeAttributeValue`, and `addBinding`.
Raw attribute extraction is in
[`xmltok_impl.c`](../expat/lib/xmltok_impl.c). Public output contracts are in
[`expat.h`](../expat/lib/expat.h).

## The start-tag pipeline

The whole start tag is first lexically validated. `storeAtts` then performs
these semantic stages before the element callback:

1. Find/intern its lexical `ELEMENT_TYPE`; obtain declaration/default metadata.
2. Extract raw `ATTRIBUTE`s with `XmlGetAttributes`; enlarge workspace if needed.
3. Intern each lexical attribute name and reject repeated QNames using the
   scratch character preceding the interned name.
4. Convert/normalize values, using this element's declared CDATA/non-CDATA type.
5. Process namespace declaration attributes immediately, establishing bindings
   and emitting namespace start callbacks. Defer expansion of ordinary prefixes.
6. Record specified-attribute count and explicit ID-attribute index.
7. Append defaults for missing attributes, also applying defaulted namespace
   declarations and emitting their namespace callbacks.
8. Expand prefixed attribute names under the now-complete scope and reject
   duplicate expanded names.
9. Clear scratch flags, terminate the name/value pointer array with null, and
   expand the element name using its prefix or current default namespace.

This ordering permits a prefix to be declared after its use in the same start
tag, and permits a DTD-defaulted namespace declaration to provide the binding.
A one-pass 'resolve every attribute as soon as seen' implementation is wrong.

`m_atts` initially holds raw descriptors and is compacted in place into
`const XML_Char *` pairs. A replacement can keep separate arrays for clarity.
Preserve explicit-attribute order, append defaults in declaration-storage order,
and omit namespace declarations from the ordinary attribute array in namespace
mode. A null terminates the array, even when it is empty.

## Normalization rules

Attribute normalization is not just string unescaping:

| Input/context | Output rule |
| --- | --- |
| Literal XML whitespace in an attribute | Normalize tab/newline/CR to space; CRLF is one line ending |
| Numeric character reference | Insert the referenced character directly; a referenced tab/newline/CR is not automatically changed into a space |
| Predefined reference | Insert its character |
| Internal general entity | Interpret replacement text in attribute-value context; whitespace and nested references follow those rules |
| CDATA attribute type | Preserve the resulting spaces; no trimming/collapse |
| Any declared non-CDATA type | Collapse runs of spaces and remove leading/trailing spaces |
| External general or unparsed entity | Reject use in an attribute value |
| Literal `<`, including markup introduced in replacement text | Reject in attribute-value scanning; a direct `&lt;` reference is permitted |

The collapse rule specifically handles space characters after normalization;
it is not a generic Unicode `trim`/`split_whitespace` operation. A direct
`&#x9;` reference can survive as a tab even for a tokenized type. Space references
participate in space collapse. Track state across entity boundaries so an entity
ending in a space and its caller beginning with a space collapse correctly.

For example, with actual source whitespace represented below using escapes:

```text
source value: "  a\tb\r\nc  "
CDATA:        "  a b c  "
NMTOKENS:     "a b c"

source value: "a&#xA;b"
both:         "a\nb"
```

Defaults are normalized at declaration time according to the declared type and
then referenced from persistent storage. A declared type's normalization applies
to explicit values too. Undeclared attributes are treated as CDATA. The raw
tokenizer's `normalized` hint avoids unnecessary normalization when possible;
it does not change the output rules.

## Attribute metadata

`XML_GetSpecifiedAttributeCount` returns the number of **pointer entries**, twice
the number of explicitly supplied ordinary attributes, excluding appended
defaults and (in namespace mode) namespace declarations. `XML_GetIdAttributeIndex`
is the name's index in this same alternating pointer array, or -1. It reports
an explicitly supplied declared ID attribute; do not infer an ID from its
spelling or insert a default ID into this metadata.

`XML_ATTR_INFO` optionally supplies original byte ranges for names and values;
value ranges exclude quote bytes. The raw workspace fills these while walking
explicit attributes; namespace declaration filtering and internal entity input
need special attention if reproducing exact metadata. Test against this source's
behavior instead of deriving byte ranges from the normalized callback values.

## Namespace modes and name forms

`XML_ParserCreateNS` or a nonnull separator argument to `XML_ParserCreate_MM`
selects namespace processing when the build provides it. `XML_NS` is the build
capability; `m_ns` is the parser's runtime mode. In ordinary mode, colons remain
part of ordinary names and `xmlns` is an ordinary attribute.

| Name case in namespace mode | Callback spelling |
| --- | --- |
| Prefixed element/attribute | `URI + separator + local` |
| Same with triplets enabled | `URI + separator + local + separator + prefix` |
| Unprefixed element with default binding | `URI + separator + local`, without a prefix component |
| Unprefixed element without default binding | Original local name |
| Unprefixed ordinary attribute | Original local name, regardless of default namespace |

A zero separator concatenates URI and local name. Combining zero separator
with triplets is a documented programming error. Separators that can occur in
names or URIs can make client-side splitting ambiguous; `addBinding` has a
specific compatibility rule rather than universal URI validation.

Example with `|` as the separator:

```xml
<r xmlns="urn:d" xmlns:p="urn:p" a="1" p:a="2"><p:x/></r>
```

```text
namespace start (NULL, "urn:d")
namespace start ("p", "urn:p")
start "urn:d|r", ["a", "1", "urn:p|a", "2", NULL]
start "urn:p|x", [NULL]
end "urn:p|x"
end "urn:d|r"
namespace end "p"
namespace end NULL
```

With triplets, the prefixed spellings become `urn:p|a|p` and `urn:p|x|p`;
the default-namespaced root stays `urn:d|r`. Namespace end order here follows
the binding list, which is built by pushing declarations; the public contract
principally requires namespace events to surround their element scope.

## Binding lifetime and callbacks

`addBinding` validates a declaration, allocates/reuses a `BINDING`, records the
old prefix binding, pushes it on the tag's binding list, changes the prefix's
current binding, and emits start-namespace when this is a real declaration.
An empty default namespace sets the active default binding to null; the frame
still remembers the previous binding for restoration.

Namespace starts precede the corresponding element start. Element end precedes
namespace ends; ends unwind bindings and restore shadowed prefixes. Empty
elements perform the same start/end namespace sequence within their one token.
Inherited bindings for external context are installed without reporting new
declarations. The implicit `xml` binding is also context, not a source `xmlns`
event.

Namespace callbacks are invoked during `storeAtts`, so application code can
change a later element handler before it is invoked. Do not snapshot every
handler at parse-call entry. Conversely, specific implementation paths cache a
handler for portions of a single converted data token; mutation behavior should
be verified at the relevant dispatch site.

## Well-formedness and reserved bindings

The scanner applies qualified-name lexical rules in namespace mode, including
colon placement and local-name starts. Semantic expansion rejects unbound
prefixes. `addBinding` enforces:

- A nondefault prefix cannot bind an empty URI (`UNDECLARING_PREFIX`).
- `xmlns` cannot be declared as a normal prefix (`RESERVED_PREFIX_XMLNS`).
- `xml` must bind exactly `http://www.w3.org/XML/1998/namespace`
  (`RESERVED_PREFIX_XML` on a wrong binding).
- That XML URI cannot be assigned to another prefix or the default namespace,
  and `http://www.w3.org/2000/xmlns/` cannot be used as an ordinary namespace
  (`RESERVED_NAMESPACE_URI`).

Empty default undeclaration `xmlns=""` is legal and gives a null URI in its
namespace callback. This is XML Namespaces 1.0 behavior; it does not permit
Namespaces 1.1 prefix undeclaration.

Expat does not generally validate a namespace URI against RFC 3986. It rejects
an occurrence of the chosen separator inside the URI when that separator is
not an RFC-3986 URI character. URI characters such as colon retain compatibility
with existing users, even though splitting can remain ambiguous. Port the
`is_rfc3986_uri_char`/`addBinding` predicate rather than replacing it with a
generic URL validator.

## Two different duplicate checks

The first check uses interned QName identity to reject `a="1" a="2"`, including
repeated namespace declarations. The second uses expanded URI/local identity:

```xml
<r xmlns:p="urn:same" xmlns:q="urn:same" p:a="1" q:a="2"/>
```

The two QNames differ but the expanded attribute names are identical, so the
document fails with `XML_ERROR_DUPLICATE_ATTRIBUTE`. Triplet prefixes must not
make them distinct. `NS_ATT` stores the URI/local hash and name, uses odd-step
probing, and marks occupied cells with a per-start-tag version. Version changes
avoid clearing the whole table for every element; wraparound reinitializes it.

Defaults participate too: an explicit attribute suppresses its own lexical
default, but a default with another QName can still collide after namespace
expansion. The default namespace never affects unprefixed ordinary attributes,
so `a` and `p:a` are normally distinct even when `p` binds the default URI.
