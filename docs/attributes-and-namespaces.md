# Expat attributes and namespaces

[Guide index](README.md)

`storeAtts` in [`xmlparse.c`](../expat/lib/xmlparse.c) connects lexical attributes, DTD defaults, namespace scope, and the start-element callback. Its processing order is essential: namespace declarations anywhere in a start tag must be visible when the element and prefixed attributes are expanded.

## Start-tag pipeline

1. Look up or create the lexical `ELEMENT_TYPE`, including its prefix identity when namespace processing is enabled.
2. Ask `XmlGetAttributes` for lexical attribute slices. Grow storage for explicit plus potentially defaulted attributes, then extract again if needed.
3. Intern each explicit name as an `ATTRIBUTE_ID`. Reject duplicate lexical names using the marker stored before its name.
4. Convert and normalize its value. The tokenizer's `normalized` flag enables a fast path; otherwise consult the element's declared type when needed and call `storeAttributeValue`.
5. Install explicit namespace declarations immediately. Remove them from the ordinary callback attribute array; defer expansion of other prefixed names until all declarations/defaults have been processed.
6. Record the specified-attribute entry count and explicit ID attribute index.
7. Append missing default values from the element declaration. A defaulted namespace declaration also installs a binding.
8. Expand prefixed attribute names and check duplicates by expanded identity. Clear per-tag attribute markers.
9. Expand the element name using its prefix or the default namespace, then return the completed callback arguments.

The C implementation reuses `m_atts`: lexical `ATTRIBUTE` records are packed into a flat pointer array of name/value pairs as processing advances. A rewrite can use separate arrays and avoid this storage trick. Preserve explicit attributes before defaults, null termination, specified-count semantics, and the distinctions between lexical and expanded identity.

## Attribute normalization

An undeclared attribute is treated as CDATA. For declared non-CDATA types, Expat collapses spaces and removes leading/trailing spaces in addition to ordinary attribute normalization. Type lookup is per element, even though attribute identifiers themselves are interned globally in the DTD.

| Value spelling or origin | CDATA value | Non-CDATA value |
| --- | --- | --- |
| `"  red   blue  "` | Leading, repeated, and trailing spaces retained | `red blue` |
| Literal tab or line ending | Space, with CRLF treated as one line ending | Space subject to collapsing/trimming |
| `"x&#10;y"` | `x`, LF, `y` | Also preserves the referenced LF |
| `"&#32; a &#32;"` | Spaces retained | Space character references participate in space collapsing/trimming |
| Internal general entity reference | Replacement text processed under attribute rules | Same, plus declared-type normalization |
| External general entity reference | Error | Error |

The defaulted value is normalized when its declaration is processed and reused later. Explicit values override defaults. `XML_GetSpecifiedAttributeCount` counts entries in the alternating name/value array, so two explicit attributes yield 4, not 2. `XML_GetIdAttributeIndex` is also an array-entry index and is `-1` when no applicable explicitly specified ID attribute is present.

## Namespace environment

Namespace processing is requested through `XML_ParserCreateNS` or a non-null separator argument to `XML_ParserCreate_MM`, with support compiled in. The default parser returns lexical names and treats namespace declaration attributes as ordinary attributes.

Namespace mode uses a `PREFIX` object for each prefix and a special default-prefix object. `addBinding` pushes a binding and remembers the previous one. Bindings introduced by a tag belong to that tag's list. Ending the tag restores earlier bindings and reuses the binding objects.

The root parser receives the implicit `xml` binding to `http://www.w3.org/XML/1998/namespace`. This inherited/implicit setup does not emit a declaration callback. Explicit declarations do emit callbacks, even when they restate an allowed binding.

## Expanded names

With separator `|`, the callback name forms are:

| Input and environment | Element name | Attribute name |
| --- | --- | --- |
| `p:item`, `xmlns:p="urn:p"` | `urn:p|item` | `urn:p|item` |
| `item`, `xmlns="urn:d"` | `urn:d|item` | `item` |
| `item`, no default namespace | `item` | `item` |

The default namespace does not apply to unprefixed attributes. With `XML_SetReturnNSTriplet` enabled, a prefixed name additionally carries its original prefix, for example `urn:p|item|p`. A default-namespaced element still has no prefix suffix.

A zero separator concatenates URI and local name without a separator; combining it with triplets is a programming error under the public contract. Choosing a separator that can appear in names or URIs makes application-side splitting ambiguous. This implementation rejects a URI containing the chosen separator only when that separator is not an RFC-3986 URI character; it preserves compatibility with applications that use URI characters such as colon. It does not otherwise perform complete URI validation or URI normalization.

## Duplicate attributes require two checks

Lexical duplicates such as `<r a="1" a="2"/>` fail whether namespaces are enabled or disabled. Namespace-aware parsing must additionally reject:

```xml
<r xmlns:p="urn:x" xmlns:q="urn:x" p:a="1" q:a="2"/>
```

The raw names differ, but both identify `{urn:x}a`. `NS_ATT` hashes the expanded URI/local-name combination and compares it after expansion. Triplet mode must not include `p` or `q` in duplicate identity, since the prefix is only a spelling choice.

This check runs after bindings are available, so declaration position within the start tag does not affect resolution. It includes defaulted prefixed attributes as well as explicit ones.

## Binding errors

`addBinding` and prefix expansion distinguish these failures:

| Condition | Error |
| --- | --- |
| Prefixed element/attribute has no binding | `XML_ERROR_UNBOUND_PREFIX` |
| Empty URI assigned to a named prefix | `XML_ERROR_UNDECLARING_PREFIX` |
| `xml` assigned to a different URI | `XML_ERROR_RESERVED_PREFIX_XML` |
| Attempt to bind `xmlns` | `XML_ERROR_RESERVED_PREFIX_XMLNS` |
| Reserved XML URI assigned to a different prefix/default namespace, or xmlns URI used as a binding | `XML_ERROR_RESERVED_NAMESPACE_URI` |
| Chosen non-URI separator appears inside URI | `XML_ERROR_SYNTAX` |

`xmlns=""` is allowed: it removes the current default namespace. The start-namespace handler receives a null prefix for a default declaration and a null URI for default-namespace removal.

## Callback ordering and lifetime

For one declaration on an empty element:

```xml
<p:r xmlns:p="urn:p"/>
```

the semantic order is:

```text
startNamespace("p", "urn:p")
startElement("urn:p|r", [])
endElement("urn:p|r")
endNamespace("p")
```

For multiple declarations, start notifications occur as bindings are added; end notifications follow the tag binding list, which is built by prepending. Callers should depend on scope nesting, not arbitrary prefix sorting. Namespace-start callbacks happen during attribute processing, so a later error in that same start tag may occur after some namespace notifications have already been delivered.

Namespace-end callbacks follow the element-end callback. This remains relevant when a handler requests suspension. Attribute arrays and expanded names are borrowed for callback use. Expanded element names can share a binding's URI buffer and are reconstructed at element end; retaining the pointer from the start event is unsafe.
