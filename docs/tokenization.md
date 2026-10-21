# Tokenization and encodings

The tokenizer is internal to the library. Its public-to-parser interface is
[`xmltok.h`](../expat/lib/xmltok.h); implementations are in
[`xmltok.c`](../expat/lib/xmltok.c), the included
[`xmltok_impl.c`](../expat/lib/xmltok_impl.c), and
[`xmltok_ns.c`](../expat/lib/xmltok_ns.c).

## Encoding as a lexical interface

`ENCODING` is a vtable, not just a character decoder. It supplies:

- Scanners for prolog, content, CDATA, and optionally ignored sections.
- Second-level scanners for attribute and entity replacement-value literals.
- Name length/matching, whitespace skipping, raw attribute extraction,
  numeric/predefined reference decoding, public-ID checking, and position update.
- Conversion to UTF-8 and UTF-16, plus minimum byte width and fast-path flags.

All input ranges and cursor advances are in bytes. Even UTF-16 scanners accept
`const char *` spans so raw offsets remain byte offsets. Converted output lengths
are in the destination code units.

`normal_encoding` extends the vtable with a 256-entry lexical class table and
multibyte naming/validity operations. This lets common ASCII-heavy paths branch
on `BT_LT`, `BT_AMP`, `BT_QUOT`, `BT_CR`, `BT_NMSTRT`, and related classes without
decoding every character into a separately allocated representation.

## Included templates and variants

`xmltok_impl.c` is included with macros such as `PREFIX`, `MINBPC`, `BYTE_TYPE`,
`CHAR_MATCHES`, and name/validity predicates. In normal builds this creates
single-byte/UTF-8 and specialized little-/big-endian UTF-16 scanner families.
`XML_MIN_SIZE` trades specialization for indirect operations and a smaller
binary. It must not change parsing semantics.

Namespace behavior also depends on encoding tables. Nonnamespace tables classify
colon as an ordinary name-start character; namespace tables retain a distinct
colon class for qualified-name checks. `xmltok_ns.c` is included with name macros
to create suffixed and unsuffixed encoding initialization/declaration entry
points. It does not implement namespace scope; that is in `xmlparse.c`.

External and internal encodings are distinct objects even when both represent
UTF-8 or native UTF-16. The internal ASCII class table treats CR specially as
already-processed data, allowing a CR introduced by a character reference in an
entity value to survive later parsing without being normalized as a raw source
line break. Reusing the external table for replacement text can silently change
characters.

## Scanner result contract

An ordinary successful token returns a positive `XML_TOK_*` and sets `next`
just after that token. Results are not simply success/error booleans:

| Result | Value / interpretation | Pointer obligation |
| --- | --- | --- |
| `XML_TOK_INVALID` | 0: malformed lexical input | `next` identifies the invalid character |
| `XML_TOK_PARTIAL` | -1: insufficient bytes to decide token | Do not rely on `next` |
| `XML_TOK_PARTIAL_CHAR` | -2: incomplete multibyte sequence | Do not rely on `next` |
| `XML_TOK_TRAILING_CR` | -3: last CR may pair with a later LF | Caller handles final/nonfinal distinction |
| `XML_TOK_NONE` | -4: no bytes | Do not rely on `next` |
| `XML_TOK_TRAILING_RSQB` | Special negative content result | Buffer trailing brackets to exclude a later `>` |
| Negated prolog token | A provisional token at span end, e.g. `-XML_TOK_NAME` or `-XML_TOK_CLOSE_PAREN` | Caller must distinguish it from the named special results |

`doProlog` waits on provisional tokens for nonfinal input, or resolves them at
final input. For example, a closing `)` at a chunk boundary could still become
`)*`, `)?`, or `)+`; a name may still continue or acquire a quantifier.
`XML_TOK_INSTANCE_START` deliberately leaves `next` at the start of `<name` so
the content scanner can rescan the element with its own grammar.

Tokenizers are bounded by `end` and have no public final-input argument. The
semantic caller decides whether an incomplete result is a request for more
input or a terminal error. Some helper scanners, such as `getAtts`, assume the
enclosing token was already validated and are not standalone untrusted-input
entry points.

## What each scanner recognizes

| Scanner | Main output |
| --- | --- |
| `contentTok` | Complete start/end/empty tags, character runs, newlines, references, comments/PIs, CDATA opener |
| `prologTok` | Declaration open/close, names and quantifiers, quoted literals, brackets, separators, parameter references, XML declarations, comments/PIs, instance start |
| `cdataSectionTok` | Literal character data/newlines and `]]>`; `<` and `&` lose markup meaning |
| `ignoreSectionTok` | Ignored conditional-section content with nested `<![`/`]]>` balancing |
| `attributeValueTok` | Data, whitespace/newlines, general and numeric references; literal `<` is invalid |
| `entityValueTok` | Data, newlines, numeric/general/parameter references for declaration-time replacement-value processing |

`scanLt` and `scanAtts` validate the whole start tag before semantic handling.
`getAtts` then enumerates `ATTRIBUTE` records: name start, value bounds excluding
quotes, and a normalization hint. It returns the total count even if the supplied
array was too small; `storeAtts` can resize and scan again. Duplicate names are
a semantic check, separate from this lexical validation.

The scanner handles XML syntax details such as mandatory attribute separators,
matching quotes, legal name starts, no `--` inside comments, PI termination and
reserved `xml` target handling, reference syntax, and illegal `]]>` in ordinary
content. Reference syntax and reference scalar legality are separate checks:
`charRefNumber` and `checkCharRefNumber` enforce the latter.

## Supported built-in input encodings

The case-insensitive name table recognizes `UTF-8`, `UTF-16`, `UTF-16BE`,
`UTF-16LE`, `ISO-8859-1`, and `US-ASCII`. An unspecified encoding uses initial
detection and defaults to UTF-8. There is no built-in UTF-32 decoder in this
table.

`INIT_ENCODING` is a temporary vtable with prolog/content scanners that call
`initScan`. It stores both the protocol-specified encoding index and a pointer
to the parser's encoding pointer. The first scan can replace that pointer with
the detected concrete vtable.

`initScan` recognizes UTF-8/UTF-16 BOMs and UTF-16 byte patterns around initial
markup. It must wait for partial BOMs and ambiguous one-byte prefixes. Rules
differ between the document entity and external text entities, especially when
a protocol encoding is supplied: byte patterns that look like a BOM can be
legitimate ISO-8859-1 external-entity data. Port its decision table or test each
branch; a single unconditional BOM-strip step is insufficient.

`processXmlDecl` applies declaration-specified encodings when there is no protocol
override. A recognized declaration must match the detected minimum character
width and UTF-16 endianness, otherwise `XML_ERROR_INCORRECT_ENCODING`. Unknown
names are offered to the application's unknown-encoding handler.

## XML declaration versus external text declaration

`scanPi` identifies the declaration-shaped token. `doParseXmlDecl` parses ordered
pseudo-attributes and quotes; `processXmlDecl` applies encoding/standalone state
and invokes callbacks.

- A document XML declaration requires `version`, may have `encoding`, and may
  have `standalone='yes'` or `'no'`, in that order.
- An external text declaration may omit `version`, must have `encoding`, and
  cannot have `standalone`.
- Declarations are permitted only at their proper initial position; ordinary
  PI placement rules do not make `<?xml ...?>` legal elsewhere.
- Version values are handled by the actual pseudo-attribute parser. There is
  no switch that turns on XML 1.1 character/name rules merely because a version
  string says `1.1`. Do not impose a stricter version-value policy without
  checking reference behavior.

The XML declaration callback's version can be absent for text declarations;
the callback contract uses standalone values -1, 0, and 1 for absent/no/yes.
Consult the actual declaration parser for accepted edge cases rather than using
presence of one callback field as a universal grammar classification algorithm.

## Validity, naming, and conversion

UTF-8 checks reject invalid continuations, overlong forms, encoded surrogate
values, values outside the Unicode scalar range, and XML-disallowed characters.
UTF-16 paths check pairing and XML character validity. Numeric references must
also reject illegal XML values, including zero, surrogates, and U+FFFE/U+FFFF.
Allowed controls are governed by the XML tables rather than a generic Unicode
decoder's acceptance rules.

Name validity is a separate question from scalar validity. `nametab.h` uses
`nmstrtPages`, `namePages`, and `namingBitmap` to represent the checkout's name
sets. The generator is [`gennmtab.c`](../expat/gennmtab/gennmtab.c). Four-byte
UTF-8 name predicates are explicitly `isNever` here: supplementary characters
can appear as data but are not accepted as names by these predicates. Replacing
the tables with a runtime's latest Unicode identifier rules changes behavior.

Conversion routines advance both source and destination pointers and return
`XML_CONVERT_COMPLETED`, `XML_CONVERT_INPUT_INCOMPLETE`, or
`XML_CONVERT_OUTPUT_EXHAUSTED`. They must not split a multibyte character or
UTF-16 surrogate pair into an incomplete output fragment. XML validity is
primarily established by scanning; conversion routines are not an independent
replacement for lexical validation.

`XML_Char` is UTF-8 `char` by default, or a UTF-16 code unit in Unicode builds.
Internal entity text uses that API representation; original document bytes can
use a different encoding. `MUST_CONVERT` permits direct callback spans for
matching UTF-8 or aligned matching UTF-16. UTF-16 alignment matters even when
the byte order matches.

## Unknown-encoding callback

`XML_Encoding.map[256]` maps a leading byte to a scalar value, -1 for malformed,
or -2/-3/-4 for a multibyte sequence. `convert(data, s)` handles multibyte
sequences. `release(data)` ends the decoder-data lifetime.

The public contract requires XML-relevant ASCII characters to retain single-byte
ASCII representations (except byte values 0x24, 0x40, 0x5C, 0x5E, 0x60, 0x7B,
0x7D, and 0x7E), at most four bytes per character, only BMP scalar values, and
no multiple encodings for one character. Built-in UTF-8/UTF-16 do not have this
BMP limitation. Read
`XmlInitUnknownEncoding`, `unknown_isInvalid`, and naming predicates alongside
the header: table validation and application converter obligations are distinct.

`handleUnknownEncoding` initializes an invalid table, asks the handler to fill
it, allocates its internal encoding object, validates/installs it, and records
release data. Failed allocation, rejected mapping, reset, and free must all
honor the release path without calling a released converter again.
