# Expat tokenization and encodings

[Guide index](README.md)

The lexical layer is defined by [`xmltok.h`](../expat/lib/xmltok.h), implemented in [`xmltok.c`](../expat/lib/xmltok.c), and specialized from [`xmltok_impl.c`](../expat/lib/xmltok_impl.c). Its interface preserves byte ranges and distinguishes malformed input from input that merely needs more bytes.

## Encoding dispatch

`ENCODING` is a table of operations, with metadata `minBytesPerChar`, `isUtf8`, and `isUtf16`. It provides:

- Scanners for prolog, content, CDATA, and, with DTD support, ignored sections.
- Separate scanners for attribute-value and entity-value literals.
- Helpers for name lengths, ASCII keyword matching, whitespace skipping, attribute extraction, numeric references, predefined entities, and public identifiers.
- Position updates and conversion to UTF-8 or UTF-16.

`INIT_ENCODING` wraps an initialization encoding and a pointer to the parser's active encoding slot. An initial scan can select the real encoding and update that slot. `m_internalEncoding` is separate: it scans entity replacement text already stored in the API's encoding.

The implementation specializes common scanner code using `PREFIX`, `BYTE_TYPE`, `MINBPC`, and character/name predicates. Normal, little-endian UTF-16, and big-endian UTF-16 variants are instantiated within `xmltok.c`; `XML_MIN_SIZE` changes how much is shared through dispatch. Namespace and ordinary encoding objects use different treatment of colon. [`xmltok_ns.c`](../expat/lib/xmltok_ns.c) is also included with different naming macros.

A rewrite can replace these preprocessor templates with generics or a decoder interface. Preserve bounded scanning and source offsets; the C macro layout itself is not a public requirement.

## Scanner contract

A scanner receives `[ptr, end)` and an output pointer for the next token position. It must never read beyond `end`. Most scanner state is local to the current call: when a token is incomplete, the caller retains it and retries later.

| Result | Meaning and caller obligation |
| --- | --- |
| Positive token | A complete lexical unit; next pointer is immediately after it |
| `XML_TOK_NONE` | Empty range; next pointer is not guaranteed to be set |
| `XML_TOK_PARTIAL` | Token lacks enough bytes to decide or finish; preserve it |
| `XML_TOK_PARTIAL_CHAR` | Incomplete multibyte character; preserve it or report a final-input error |
| `XML_TOK_INVALID` | Malformed token; next pointer identifies the offending location |
| `XML_TOK_TRAILING_CR` | CR may combine with a subsequent LF; defer on non-final input |
| `XML_TOK_TRAILING_RSQB` | Trailing `]` or `]]` may become an illegal `]]>` in content |
| Other negative prolog token | An unfinished token whose lexical class is known, such as trailing prolog whitespace; `doProlog` has special final-input handling |

Do not assume that every negative result means the same thing or that every result sets the output pointer. `doContent` initializes its next pointer defensively; `accountingDiffTolerated` rejects inappropriate token classes before touching the range.

Partial input may be a valid prefix of a document. Only the parser knows whether more input is possible. On final input, it translates incomplete tokens to errors such as `XML_ERROR_UNCLOSED_TOKEN` or `XML_ERROR_PARTIAL_CHAR`. Context matters: an unfinished CDATA section has its own error, while incomplete entity markup can produce an entity-boundary error.

## Lexical contexts

| Context | Main entry | Responsibilities |
| --- | --- | --- |
| Element content | `contentTok` | Whole start/end tags, text runs, references, comments, PIs, CDATA opener, and forbidden `]]>` detection |
| Prolog and DTD | `prologTok` | Declaration openers, names, literals, punctuation, parameter references, group quantifiers, and document-element transition |
| Attribute value | `attributeValueTok` | Text, references, literal whitespace, and invalid literal markup |
| Entity declaration value | `entityValueTok` | Text, character references, general references retained as text, and parameter references for semantic processing |
| CDATA | `cdataSectionTok` | Text and line endings until `]]>`; references and `<` are ordinary data |
| Ignored DTD section | `ignoreSectionTok` | Balanced nested conditional-section delimiters without interpreting ignored content as declarations |

`scanLt` dispatches markup after `<`. `scanAtts` checks attribute syntax as part of recognizing the complete start tag. `getAtts` subsequently extracts name/value slices and a normalization hint from that validated token. It reports the total attribute count even if the supplied array is too small, allowing the parser to grow the array and extract again.

This two-pass structure avoids building semantic objects for a half-read start tag. A rewrite may combine validation and extraction, but must withhold start-element delivery until the complete tag, all values, defaults, and namespace checks are resolved.

## Character validity and names

Byte classes separate XML punctuation, whitespace, name characters, illegal XML characters, malformed bytes, and multibyte lead/continuation bytes. UTF-8 predicates reject overlong encodings, surrogate encodings, and values outside the supported scalar range. UTF-16 scanners recognize surrogate pairs and reject malformed sequences. Numeric references are separately range-checked by `charRefNumber` and `checkCharRefNumber`.

Name acceptance is not interchangeable with a language runtime's Unicode identifier predicate. [`nametab.h`](../expat/lib/nametab.h) supplies `namePages`, `nmstrtPages`, and bitmap data used by the encoding predicates. Keep these acceptance rules as an explicit compatibility decision. Accepting additional characters using a newer Unicode or XML name definition changes behavior.

Namespace-aware scanners enforce QName shape in the relevant element/attribute contexts: a colon separates two valid name parts, and a second colon is rejected. Ordinary parsing treats colon as part of an XML name. Namespace binding and reserved-name rules belong to `xmlparse.c`, after lexical recognition.

Other lexical checks include legal comment termination, PI target rules and termination, quoting and separators in start tags, and public-identifier character restrictions. Generic string splitting will miss both malformed-input cases and split delimiters.

## Encoding selection

Built-in encoding names are matched case-insensitively: `UTF-8`, `UTF-16`, `UTF-16BE`, `UTF-16LE`, `US-ASCII`, and `ISO-8859-1`. With no supplied encoding, UTF-8 is the fallback, subject to leading-byte detection.

`initScan` considers both the externally supplied encoding and whether it is scanning a document prolog or external general-entity content. It recognizes BOMs and leading zero-byte patterns that indicate UTF-16. Detection can itself return partial when only one byte or an incomplete UTF-8 BOM is present. Explicitly encoded external entities have special cases so bytes that resemble a BOM are not always consumed as one.

`processXmlDecl` handles the XML declaration or external text declaration after lexical recognition. `doParseXmlDecl` enforces pseudo-attribute order and structure: the document declaration requires version; a text declaration requires encoding and cannot specify standalone. The callback can receive the version string without Expat switching to a different grammar version. Do not infer XML 1.1 parsing behavior from the version text alone.

If no protocol encoding was supplied, a declaration can select the encoding, subject to consistency with the already inferred minimum character width and UTF-16 byte order. A supplied protocol encoding takes precedence over using the declaration to change the decoder. Unknown names use the unknown-encoding callback.

## Input encoding and API encoding

Input always arrives through `char *` byte ranges, even in a Unicode build. Callback string encoding is fixed by the build: UTF-8 for ordinary `XML_Char`, or UTF-16 for the Unicode variants. UTF-16 input does not imply UTF-16 callbacks.

The conversion interface advances source and destination pointers and returns `XML_CONVERT_COMPLETED`, `XML_CONVERT_INPUT_INCOMPLETE`, or `XML_CONVERT_OUTPUT_EXHAUSTED`. Conversion must preserve whole output characters at buffer boundaries; never split a UTF-8 sequence or UTF-16 surrogate pair merely because the output buffer filled.

`MUST_CONVERT` checks whether direct callbacks are possible. In UTF-16 configurations it also considers alignment. Text conversion may emit multiple callbacks from a fixed-size scratch buffer. Token validation and conversion have different responsibilities: conversion helpers are often operating on input already checked by a scanner and should not be treated as a standalone validating XML decoder.

## Normalization depends on origin and context

| Input context | Result |
| --- | --- |
| Literal CR or CRLF in document text | One LF in character data |
| Literal line endings in comments and PI data | Normalized by reporting helpers |
| Literal whitespace in attribute values | Normalized to spaces; non-CDATA types also collapse and trim spaces |
| Character reference in an attribute | The referenced character is preserved; `&#10;` produces LF, not the space produced by a literal line break |
| Default-handler text | Original spelling and line endings, converted to API encoding; BOM omitted |
| General reference in an entity declaration value | Retained for processing when the entity is used |

The internal ASCII table classifies CR as `BT_S` instead of `BT_CR`. This prevents a CR introduced by a character reference into stored replacement text from being line-normalized again during content expansion. Attribute normalization still applies its own rules. A single global whitespace or newline preprocessing pass cannot reproduce these distinctions.

## Unknown encodings

`XML_UnknownEncodingHandler` fills an `XML_Encoding` structure:

- `map[byte] >= 0` supplies a single-byte scalar value.
- `-1` means an invalid byte.
- `-2`, `-3`, or `-4` denotes the length of a multibyte sequence; `convert(data, bytes)` supplies the scalar or reports failure.
- `release(data)` releases application encoding state when Expat no longer needs it, including failure paths after the callback supplied it.

The public contract restricts these encodings: required ASCII XML characters must retain their single-byte ASCII representations, sequences use at most four bytes, supported values stay at or below `0xFFFF` without surrogates, and characters cannot have multiple distinct encodings. These restrictions do not limit built-in UTF-8/UTF-16 in the same way. `XmlInitUnknownEncoding` constructs scanner/conversion tables from the description; `handleUnknownEncoding` manages ownership and cleanup.
