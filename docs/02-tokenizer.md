# 2. The tokenizer: `xmltok.c`, `xmltok_impl.c`, `xmltok_ns.c`

The tokenizer turns raw bytes in the document's encoding into tokens. It has
no state of its own and never allocates memory. It knows nothing about the
DTD, entities, or handlers.

## 2.1 The scanner contract

Every scanner has the signature (`xmltok.h`):

```c
typedef int (PTRCALL *SCANNER)(const ENCODING *enc,
                               const char *ptr, const char *end,
                               const char **nextTokPtr);
```

It looks at `[ptr, end)`, recognizes **one** token at `ptr`, and returns its
code. The comment in `xmltok.h` spells out the protocol:

| Return value | Meaning | `*nextTokPtr` |
|--------------|---------|---------------|
| `> 0` | A complete token of that type | end of the token |
| `XML_TOK_NONE` (-4) | `ptr == end`, nothing to scan | not set |
| `XML_TOK_PARTIAL` (-1) | The bytes so far are a valid *prefix* of a token; more input is needed | not set |
| `XML_TOK_PARTIAL_CHAR` (-2) | The input ends in the middle of a multibyte character | not set |
| `XML_TOK_INVALID` (0) | Not well-formed | points at the offending character |
| `XML_TOK_TRAILING_CR` (-3) | A CR at the very end; it might be the first half of CRLF | |
| `XML_TOK_TRAILING_RSQB` (-5) | `]` or `]]` at the end; it might start an illegal `]]>` | |
| `-tok` (prolog only) | A token that is complete *if no more input follows*, such as a name or whitespace that runs up to `end` | end |

This contract is what makes **incremental parsing** possible. The parser
never needs to keep tokenizer state across `XML_Parse` calls. When it gets
`PARTIAL` and more input is coming (`haveMore`), it records how far it
consumed. Next time it calls the same scanner again from the start of the
unfinished token, with more bytes available. The cost is that a large token
split across many small buffers gets rescanned many times. "Reparse deferral"
(see [07](07-memory-and-security.md#74-reparse-deferral)) limits that.

Text runs are not split into individual characters. `contentTok` returns
`XML_TOK_DATA_CHARS` for the longest run of ordinary characters and stops at
`<`, `&`, `]`, CR, LF, or an invalid byte. So the parser usually calls the
character-data handler once per run, not once per character.

## 2.2 Token codes

Defined in `xmltok.h`. Grouped by the scanner that produces them:

* **Content** (`XmlContentTok`): `START_TAG_WITH_ATTS`, `START_TAG_NO_ATTS`,
  `EMPTY_ELEMENT_WITH_ATTS`, `EMPTY_ELEMENT_NO_ATTS`, `END_TAG`,
  `DATA_CHARS`, `DATA_NEWLINE`, `CDATA_SECT_OPEN`, `ENTITY_REF`, `CHAR_REF`,
  `PI`, `XML_DECL`, `COMMENT`, `BOM`.

  The parser handles *WITH_ATTS* and *NO_ATTS* the same way (both go
  through `storeAtts`, because defaulted attributes and namespace expansion
  of the element name are needed either way). The distinction is
  informational.
* **Prolog** (`XmlPrologTok`): `PROLOG_S`, `DECL_OPEN` (`<!foo`),
  `DECL_CLOSE`, `NAME`, `NMTOKEN`, `POUND_NAME` (`#PCDATA`, `#REQUIRED`…),
  `OR`, `PERCENT`, `OPEN/CLOSE_PAREN`, `OPEN/CLOSE_BRACKET`, `LITERAL`,
  `PARAM_ENTITY_REF`, `INSTANCE_START` (first `<name` = root element starts),
  `NAME_QUESTION/ASTERISK/PLUS`, `COND_SECT_OPEN/CLOSE`,
  `CLOSE_PAREN_QUESTION/ASTERISK/PLUS`, `COMMA`, `PREFIXED_NAME`
  (namespace builds only).
* **CDATA section** (`XmlCdataSectionTok`): `DATA_CHARS`, `DATA_NEWLINE`,
  `CDATA_SECT_CLOSE`.
* **Ignore section** (`XmlIgnoreSectionTok`, `XML_DTD` only): `IGNORE_SECT`.
* **Literals** (`XmlAttributeValueTok`, `XmlEntityValueTok`): these split an
  already-delimited literal into `DATA_CHARS`, `DATA_NEWLINE`,
  `ATTRIBUTE_VALUE_S`, `ENTITY_REF`, `CHAR_REF`, `PARAM_ENTITY_REF`.

The scanner "states" are just indices into `enc->scanners[]`:
`XML_PROLOG_STATE`, `XML_CONTENT_STATE`, `XML_CDATA_SECTION_STATE`, and
`XML_IGNORE_SECTION_STATE`. The *parser* chooses which one to call; the
tokenizer does not track modes itself.

## 2.3 `struct encoding`: the vtable

`xmltok.h:172`:

```c
struct encoding {
  SCANNER scanners[XML_N_STATES];              // prolog/content/cdata/ignore
  SCANNER literalScanners[XML_N_LITERAL_TYPES];// attribute value / entity value
  int  (*nameMatchesAscii)(enc, ptr1, end1, asciiStr);
  int  (*nameLength)(enc, ptr);
  const char *(*skipS)(enc, ptr);
  int  (*getAtts)(enc, ptr, attsMax, ATTRIBUTE *atts);
  int  (*charRefNumber)(enc, ptr);
  int  (*predefinedEntityName)(enc, ptr, end);   // &lt; &gt; &amp; &quot; &apos;
  void (*updatePosition)(enc, ptr, end, POSITION *);
  int  (*isPublicId)(enc, ptr, end, &badPtr);
  enum XML_Convert_Result (*utf8Convert)(enc, &from, fromLim, &to, toLim);
  enum XML_Convert_Result (*utf16Convert)(enc, &from, fromLim, &to, toLim);
  int  minBytesPerChar;   // 1 or 2
  char isUtf8;            // input already in internal UTF-8?
  char isUtf16;           // input already in native-endian UTF-16?
};
```

Concrete encodings extend it by embedding it as the **first member**, so a
pointer to the outer struct can be used as an `ENCODING *`:

```c
struct normal_encoding {          // xmltok.c:187
  ENCODING enc;
  unsigned char type[256];        // BT_* class of each byte (or of the low
                                  // byte, for UTF-16 code units < 0x100)
  /* XML_MIN_SIZE only: byteType, isNameMin, isNmstrtMin, byteToAscii, charMatches */
  isName2/3/4, isNmstrt2/3/4, isInvalid2/3/4   // multibyte-sequence predicates
};

struct unknown_encoding {         // xmltok.c:1309
  struct normal_encoding normal;
  CONVERTER convert;  void *userData;
  unsigned short utf16[256];
  char utf8[256][4];
};
```

`AS_NORMAL_ENCODING(enc)` and `AS_UNKNOWN_ENCODING(enc)` are the downcasts.

### Encoding instances

`xmltok.c` defines one static `const` instance of each of these (plus an
`_ns` twin when `XML_NS` is on):

| Instance | Scanners | Converters | Byte table |
|----------|----------|------------|------------|
| `utf8_encoding` | `normal_*` | `utf8_toUtf8` (`memcpy`), `utf8_toUtf16` | `asciitab` + `utf8tab` |
| `internal_utf8_encoding` | `normal_*` | same | `iasciitab` + `utf8tab` |
| `latin1_encoding` | `normal_*` | `latin1_toUtf8/16` | `asciitab` + `latin1tab` |
| `ascii_encoding` | `normal_*` | `ascii_toUtf8`, `latin1_toUtf16` | `asciitab` + all `BT_NONXML` |
| `little2_encoding` / `big2_encoding` | `little2_*` / `big2_*` | `*_toUtf8`, `*_toUtf16` | `asciitab` + `latin1tab` (low byte only) |
| `internal_little2/big2_encoding` | (same) | (same) | `iasciitab` + `latin1tab` |

The **internal encodings** are used to tokenize text Expat has already
stored, mainly entity replacement text, which is kept in `XML_Char` form.
`XmlGetUtf8InternalEncoding()` and `XmlGetUtf16InternalEncoding()`
(`xmltok_ns.c`) return the one that matches `XML_Char` and the host byte
order.

## 2.4 Byte classification

The core of every scanner is `switch (BYTE_TYPE(enc, ptr))` over the `BT_*`
classes in `xmltok_impl.h`:

```
BT_NONXML BT_MALFORM BT_LT BT_AMP BT_RSQB BT_LEAD2 BT_LEAD3 BT_LEAD4 BT_TRAIL
BT_CR BT_LF BT_GT BT_QUOT BT_APOS BT_EQUALS BT_QUEST BT_EXCL BT_SOL BT_SEMI
BT_NUM BT_LSQB BT_S BT_NMSTRT BT_COLON BT_HEX BT_DIGIT BT_NAME BT_MINUS
BT_OTHER BT_NONASCII BT_PERCNT BT_LPAR BT_RPAR BT_AST BT_PLUS BT_COMMA BT_VERBAR
```

* For single-byte encodings, `BYTE_TYPE` is `type[(unsigned char)*p]`: one
  table lookup.
* For UTF-16 it is `hi == 0 ? type[lo] : unicode_byte_type(hi, lo)`. The
  second function maps surrogates to `BT_LEAD4`/`BT_TRAIL`, `U+FFFE`/`U+FFFF`
  to `BT_NONXML`, and everything else to `BT_NONASCII`.
* For UTF-8 lead bytes the class is `BT_LEAD2/3/4`. The scanner then calls
  `IS_INVALID_CHAR(enc, p, n)` (rejecting overlong forms, surrogates, and
  `U+FFFE`/`U+FFFF`, `xmltok.c:107-130`). For names it also calls
  `IS_NAME_CHAR(enc, p, n)` / `IS_NMSTRT_CHAR(enc, p, n)`.
* `BT_NONASCII` means "could be a name character, look it up". The lookup goes
  through `nametab.h`: a two-level table where `nmstrtPages[hi]` or
  `namePages[hi]` selects a 256-bit page in `namingBitmap`, and the low byte
  selects the bit (`UCS2_GET_NAMING`, `UTF8_GET_NAMING2/3` in `xmltok.c`).
  The tables encode the XML 1.0 (4th edition) name-character classes, which
  are defined only for the BMP. That is why `utf8_isName4` and
  `utf8_isNmstrt4` are `isNever`.

The namespace twins differ in exactly one byte class: in `_ns` encodings `:`
is `BT_COLON`, so `scanLt` and `prologTok` can see prefixes and return
`XML_TOK_PREFIXED_NAME`. Elsewhere `:` is `BT_NMSTRT`. This happens at table
construction time with `#define BT_COLON BT_NMSTRT` around the `asciitab.h`
include.

## 2.5 How `xmltok_impl.c` is instantiated

Each scanner in `xmltok_impl.c` is written in terms of:

| Macro | `normal_` | `little2_` / `big2_` |
|-------|-----------|----------------------|
| `PREFIX(x)` | `normal_x` | `little2_x` / `big2_x` |
| `MINBPC(enc)` | `1` (or `enc->minBytesPerChar` under `XML_MIN_SIZE`) | `2` |
| `BYTE_TYPE(enc,p)` | table lookup | hi/lo check |
| `BYTE_TO_ASCII(enc,p)` | `*p` | low byte if hi == 0, else -1 |
| `CHAR_MATCHES(enc,p,c)` | `*p == c` | `hi == 0 && lo == c` |
| `IS_NAME_CHAR(enc,p,n)` | `isName2/3/4` vtable call | `0` (non-BMP is handled via `BT_NONASCII` + `*_MINBPC`) |
| `IS_NAME_CHAR_MINBPC(enc,p)` | `0` | `UCS2_GET_NAMING(namePages, hi, lo)` |

`VTABLE`/`VTABLE1` (`xmltok.c:70`) then build the `ENCODING` initializer
from `PREFIX(prologTok)`, `PREFIX(contentTok)`, and so on.

The main scanners in `xmltok_impl.c`:

| Function | Purpose |
|----------|---------|
| `contentTok` (`:823`) | Content mode. Dispatches on the first byte to `scanLt` (`<`), `scanRef` (`&`), newline handling, `]]>` detection, otherwise a run of data characters |
| `scanLt` (`:725`) | After `<`: comment, CDATA, PI, end tag, or start tag. For start tags it scans attribute syntax via `scanAtts` and decides `*_WITH_ATTS` vs `*_NO_ATTS` and `EMPTY_ELEMENT` vs `START_TAG` |
| `scanAtts` | Checks attribute syntax: names, `=`, quoted values, no `<` in values. Values are **not** decoded here |
| `prologTok` (`:1017`) | Prolog/DTD mode: literals, `<!`, `<?`, `%name;`, punctuation, names and nmtokens with occurrence suffixes, `INSTANCE_START` |
| `scanDecl`, `scanPi`, `scanComment`, `scanCdataSection`, `scanEndTag`, `scanRef`, `scanCharRef`, `scanHexCharRef`, `scanPercent`, `scanPoundName`, `scanLit` | Sub-scanners |
| `attributeValueTok`, `entityValueTok` (`:1261`, `:1330`) | Split the *inside* of a literal into data, references, and whitespace |
| `ignoreSectionTok` | Skip a `<![IGNORE[ … ]]>` section, tracking nested `<![` … `]]>` |
| `getAtts` (`:1509`) | Second pass over a start tag that has already been validated. Fills `ATTRIBUTE{name, valuePtr, valueEnd, normalized}` for up to `attsMax` attributes and returns the true count. `normalized` is cleared if the value contains `&`, a CR/LF, or whitespace that is not a single space, so the parser knows whether a slow normalizing pass is needed |
| `charRefNumber`, `predefinedEntityName`, `nameMatchesAscii`, `nameLength`, `skipS`, `isPublicId`, `updatePosition` | Small helpers exposed through the vtable |

`getAtts` relies on `scanAtts` having already checked well-formedness. It is
a simple state machine (`other`/`inName`/`inValue`) that does no error
checking. This "validate once, extract fast" split is common in Expat.

## 2.6 Transcoding

`XmlUtf8Convert` / `XmlUtf16Convert` copy as much as fits from
`[*fromP, fromLim)` into `[*toP, toLim)`, update both pointers, and return:

* `XML_CONVERT_COMPLETED`: all input was consumed.
* `XML_CONVERT_INPUT_INCOMPLETE`: stopped before a partial trailing
  character.
* `XML_CONVERT_OUTPUT_EXHAUSTED`: the output buffer is full.

They never split a multibyte character. `utf8_toUtf8` uses
`_INTERNAL_trim_to_complete_utf8_characters` (`xmltok.c:328`) to cut both
`fromLim` (incomplete input) and the copy length (limited output) back to a
character boundary. Callers loop on `OUTPUT_EXHAUSTED`. `poolAppend` grows
the pool; `doContent` and `reportDefault` flush `m_dataBuf` to the handler and
continue.

On the parser side, `MUST_CONVERT(enc, s)` (`xmlparse.c`) decides whether
conversion is needed at all. For a UTF-8 document and a `char` API it is
false, so **character data is passed to handlers as pointers straight into the
input buffer** with no copy. With `XML_UNICODE`, UTF-16 input in host byte
order that is 2-byte aligned is also passed through without a copy.

## 2.7 Encoding detection

At creation, `XmlInitEncoding(&m_initEncoding, &m_encoding, name)`
(`xmltok_ns.c`) does not pick a real encoding yet. It stores the index of the
externally specified ("protocol") encoding, if there is one, in
`initEnc.isUtf16` (reused for binary compatibility, see `INIT_ENC_INDEX`).
It also installs **detecting scanners**, `initScanProlog` / `initScanContent`,
as `m_encoding`.

The first call to `XmlPrologTok` (or `XmlContentTok` for external entities)
goes to `initScan` (`xmltok.c:1532`). It looks at the first two to four bytes:

| Bytes | Decision |
|-------|----------|
| `FE FF` | UTF-16BE BOM → `XML_TOK_BOM` |
| `FF FE` | UTF-16LE BOM → `XML_TOK_BOM` |
| `EF BB BF` | UTF-8 BOM → `XML_TOK_BOM` |
| `3C 00` | UTF-16LE without BOM (`<` followed by NUL) |
| `00 xx` | UTF-16BE without BOM |
| `xx 00` | UTF-16LE (document entity only) |
| anything else | the protocol encoding, or UTF-8 if none |

There are many exceptions for external parsed entities whose declared encoding
makes BOM-like bytes legal data. `initScan` then writes the chosen `ENCODING *`
through `INIT_ENCODING.encPtr` (pointing at `parser->m_encoding`), so later
calls go directly to the real scanners, and it forwards the current call to
that encoding.

Next, if the document starts with `<?xml … ?>`, the parser's `processXmlDecl`
calls `XmlParseXmlDecl` → `doParseXmlDecl` (`xmltok.c:1152`). That function
parses the pseudo-attributes (`version`, `encoding`, `standalone`) and looks up
the encoding name with `findEncoding`. The parser accepts the declared
encoding only if it agrees with what was detected (same `minBytesPerChar`, and
the same endianness for UTF-16); otherwise it reports
`XML_ERROR_INCORRECT_ENCODING`. An unrecognized name goes to the user's
unknown-encoding handler.

## 2.8 User-defined ("unknown") encodings

If the encoding name is not one of ISO-8859-1, US-ASCII, UTF-8, UTF-16,
UTF-16BE, or UTF-16LE, `handleUnknownEncoding` (`xmlparse.c`) calls the
application's `XML_UnknownEncodingHandler`. The handler fills an
`XML_Encoding` (`expat.h:527`):

```c
typedef struct {
  int map[256];   // >=0: byte is that code point; -1: invalid;
                  // -2..-4: lead byte of a 2..4-byte sequence
  void *data;
  int (*convert)(void *data, const char *s);  // decodes multibyte sequences
  void (*release)(void *data);
} XML_Encoding;
```

`XmlInitUnknownEncoding` (`xmltok.c:1401`) builds a `struct unknown_encoding`
in memory the parser allocates (size from `XmlSizeOfUnknownEncoding()`). It
copies `latin1_encoding` as the base and then:

* rejects maps that change the meaning of ASCII characters that are
  significant to XML,
* fills `type[]` with `BT_*` classes derived from each mapped code point
  (`BT_LEAD2..4` for multibyte leads),
* precomputes `utf8[256][4]` and `utf16[256]` for single-byte characters, and
* installs `unknown_isName`, `unknown_isNmstrt`, `unknown_isInvalid`,
  `unknown_toUtf8`, and `unknown_toUtf16`, which call `convert` for multibyte
  sequences.

So user encodings are limited to ASCII-compatible encodings with at most four
bytes per character. They reuse the `normal_` (one byte per unit) scanners.

## 2.9 Position tracking

The tokenizer does not count lines. The parser calls
`XmlUpdatePosition(enc, from, to, &m_position)` *lazily*:

* at the end of each `XML_Parse*` call, from `m_positionPtr` to the point it
  consumed up to, and
* when the application asks for `XML_GetCurrentLineNumber` / `ColumnNumber`,
  up to `m_eventPtr`.

`updatePosition` walks the bytes, counting CR, LF, and CRLF as one line break
and multibyte characters as one column. Lines and columns are 0-based
internally; the API adds 1 to the line number.

## 2.10 Appendix: lexical rules a reimplementation must match

These come from `xmltok_impl.c`. "Name start" means `BT_NMSTRT`, `BT_HEX`,
or a non-ASCII character in `nmstrtPages`. "Name char" adds `BT_DIGIT`,
`BT_NAME` (`.`, U+00B7), `BT_MINUS`, and `namePages` (plus `BT_COLON` in
some namespace contexts). `S` means space, TAB, CR, or LF.

**Content (`contentTok`)**

* A data run stops before `<`, `&`, CR, LF, `]]>`, an invalid byte, or an
  incomplete multibyte character at the end. A lone `]` or `]]` without `>`
  is data. `]]>` is `XML_TOK_INVALID` at the second `]`.
* CR or CRLF → one `DATA_NEWLINE`. A CR at the very end of the input →
  `TRAILING_CR`. `]`/`]]` at the end → `TRAILING_RSQB`.

**Start tag (`scanLt` / `scanAtts`)**

* `<` + name start. The name continues while there are name chars. In
  namespace builds at most **one** `:` is allowed, and it must be followed by
  a name start.
* After the name: `>`, `/>`, or S. After S: more S, `>`, `/>`, or an
  attribute name.
* Attribute: name, optional S, `=`, optional S, `"` or `'`, value, the same
  quote. Inside the value: `<` is invalid. `&` must start a well-formed
  reference (`&name;`, `&#digits;`, `&#xhex;`). The value is not decoded at
  this stage.
* **After the closing quote, S, `>`, or `/>` is required.** `a="1"b="2"` is
  invalid at `b`.
* Attribute names follow the same one-colon rule.

**End tag (`scanEndTag`)**: `</` + name start, then name chars (any number of
colons, since the end tag only has to match the start tag byte for byte),
then optional S, then `>`.

**Comment**: `<!--` … `-->`. Any `--` not immediately followed by `>` is
invalid at the character after the `--`, so `<!-- a--->` is invalid.

**PI (`scanPi`)**: `<?` + name, then `?>` or S … `?>`. A target of exactly
`xml` gives `XML_TOK_XML_DECL`. A three-letter target spelling `xml` with any
uppercase letter is **invalid**. Other targets that merely start with `xml`
(`xml-stylesheet`) are fine.

**CDATA**: `<![CDATA[` (exact case) … `]]>`; inside, newlines are tokenized
separately.

**References**: `&` + name + `;` → `ENTITY_REF`. `&#` + decimal digits +
`;`, or `&#x` + hex digits + `;` (**lowercase `x` only**), → `CHAR_REF`. The
character's validity is checked later by `XmlCharRefNumber`:
`checkCharRefNumber` rejects surrogates, U+FFFE/U+FFFF, values ≥ 0x110000,
and C0 controls other than TAB, LF, and CR.

**Prolog (`prologTok`)**

* Literal `"…"` / `'…'`: may contain anything valid (including `<`, `&`, `%`;
  they are decoded later). **It must be followed by S, `>`, `%`, or `[`**,
  or by the end of the input (→ `-LITERAL`). Otherwise the token is invalid.
* `<!` + letters → `DECL_OPEN`, ending at S or `%`. `<!ENTITY%` is invalid:
  `%` must not be followed by S or another `%` there. `<!--` → comment.
  `<![` → `COND_SECT_OPEN`.
* `%` + name + `;` → `PARAM_ENTITY_REF`. A `%` followed by S → `PERCENT` (in
  `<!ENTITY % name`).
* `#` + name → `POUND_NAME`, which must end at S, `)`, `>`, `%`, or `|`.
* Names and nmtokens end at `>`, `)`, `,`, `|`, `[`, `%`, or S. A `?`, `*`,
  or `+` straight after a *name* gives `NAME_QUESTION/ASTERISK/PLUS`; after an
  *nmtoken* it is invalid.
* `)` may be followed directly by `*`, `?`, or `+`
  (`CLOSE_PAREN_ASTERISK`/…); otherwise the next character must be S, `>`,
  `,`, `|`, or `)`.
* `]]>` → `COND_SECT_CLOSE`, a lone `]` → `CLOSE_BRACKET`.
* `<` + name start → `INSTANCE_START`. The token is empty: `nextTokPtr` is
  set back to the `<`, so content parsing starts at the root's `<`.
* Whitespace runs are one `PROLOG_S` token. The scanner avoids splitting a
  CRLF pair at the end of the buffer.

**Negative ("maybe incomplete") tokens:** at the end of the input, the
prolog scanner returns `-tok` for tokens that could continue (`PROLOG_S`,
names, nmtokens, `POUND_NAME`, `CLOSE_PAREN`, `CLOSE_BRACKET`, `LITERAL`).
`doProlog` treats `tok < 0` as "need more data" while more input is coming,
and otherwise as the complete token `-tok`.

**UTF-16 alignment:** for 2-byte encodings, every scanner first rounds `end`
down to a multiple of 2. An odd byte at the end gives `PARTIAL` (or is
treated as not yet available).

**XML / text declaration (`doParseXmlDecl`)**:
`<?xml` S `version` Eq quoted `[A-Za-z0-9._-]*`, then optionally S `encoding`
Eq quoted value starting with a letter, then optionally S `standalone` Eq
`"yes"` or `"no"`, then optional S and `?>`. In a **text declaration**
(external entity), `version` is optional, `encoding` is **required**, and
`standalone` is forbidden. Pseudo-attribute names are case-sensitive.
Whitespace around `=` is allowed. Values may use either quote.
