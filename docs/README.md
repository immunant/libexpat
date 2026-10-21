# libexpat internals: a map of `expat/lib/`

These notes describe how the C sources under `expat/lib/` work: the overall
architecture, the main data structures, how a document flows through the code,
and why things are built the way they are. They were written against
**Expat 2.7.4** (`XML_MAJOR/MINOR/MICRO_VERSION` in `expat/lib/expat.h:1083`).

Line references such as `xmlparse.c:663` refer to files in `expat/lib/` at
that version. Expect them to drift slightly as the code changes.

## Reading order

| # | Document | What it covers |
|---|----------|----------------|
| 1 | [01-architecture.md](01-architecture.md) | The three layers, a file-by-file map, build-time configuration macros, and the preprocessor "template" tricks |
| 2 | [02-tokenizer.md](02-tokenizer.md) | `xmltok.c` / `xmltok_impl.c` / `xmltok_ns.c`: the `ENCODING` vtable, byte-type tables, token codes, partial tokens, encoding detection, user-defined encodings |
| 3 | [03-prolog-state-machine.md](03-prolog-state-machine.md) | `xmlrole.c`: the DTD/prolog grammar as a state machine of function pointers that turns tokens into *roles* |
| 4 | [04-parser-core.md](04-parser-core.md) | `xmlparse.c`: `XML_ParserStruct`, *processors*, the input buffer, `callProcessor` and the re-enter loop, suspend/resume, content parsing, attributes, namespaces |
| 5 | [05-dtd-and-entities.md](05-dtd-and-entities.md) | The `DTD` object, entity declaration and expansion without recursion, external entity sub-parsers, parameter entities, content-model scaffolding, default attributes |
| 6 | [06-data-structures.md](06-data-structures.md) | The internal containers: open-addressing hash table, string pool, tag stack, namespace bindings, free lists, attribute flag tricks |
| 7 | [07-memory-and-security.md](07-memory-and-security.md) | Pluggable allocators, the allocation tracker, billion-laughs accounting, hash-flooding protection (SipHash + salt), reparse deferral, overflow-checking idioms, removing recursion |
| 8 | [08-walkthrough.md](08-walkthrough.md) | A complete trace of a small document from `XML_ParserCreate` to `XML_ParserFree` |
| 9 | [09-behavioral-spec.md](09-behavioral-spec.md) | **The observable contract** for a reimplementation: lifecycle and state table, every event and its arguments, text normalization rules, every well-formedness check with its error code, entity/DTD rules, position reporting, encoding resolution, limits, build variants, quirks (⚑) to preserve, and the test suites to check against |
| 10 | [10-api-reference.md](10-api-reference.md) | Every exported function and type, with argument checks, return values, and side effects |
| 11 | [11-rust-design.md](11-rust-design.md) | **Design proposal** for a safe Rust reimplementation that stays drop-in compatible through a C ABI layer: crate layout, borrowing model for re-entrant handlers, DTD lending, allocator-generic memory, FFI details, and a variant of the plan that starts from c2rust output |

Appendices with exhaustive tables: the full prolog grammar transition table
is in [03 §3.5](03-prolog-state-machine.md#35-appendix-complete-transition-table),
and the lexical rules of every scanner are in
[02 §2.10](02-tokenizer.md#210-appendix-lexical-rules-a-reimplementation-must-match).

## If you are reimplementing Expat

Read in this order: **09 → 10 → 02 §2.10 → 03 §3.5 → 05 → 04 → 07**, then
use 06 and 08 as needed. Checklist of things that are easy to miss:

1. **Incremental parsing with a stateless tokenizer.** Any split of the input
   must give the same events. Partial tokens are rescanned. Errors about
   incomplete input appear only on the final buffer. Keep CR and `]`/`]]` at
   the end of a buffer for the next call ([02 §2.1](02-tokenizer.md#21-the-scanner-contract), [04 §4.4](04-parser-core.md#44-input-buffering)).
2. **Suspend and resume at any token**, including in the middle of entity
   expansion, so all expansion state must live in the parser object, not on
   the call stack ([04 §4.7](04-parser-core.md#47-parsing-status-suspend-resume-abort), [05 §5.4](05-dtd-and-entities.md#54-expanding-internal-entities-without-recursion)).
3. **No recursion driven by input**: entity expansion and content models
   ([07 §7.5](07-memory-and-security.md#75-removing-recursion)).
4. **Amplification and allocation limits**, with the exact defaults and the
   direct/indirect byte accounting ([07 §7.1–7.2](07-memory-and-security.md)).
5. **Keyed hashing** for every table that holds names from the input
   ([07 §7.3](07-memory-and-security.md#73-hash-flooding-protection)).
6. **The default-handler fallback** and the rule that
   `XML_SetDefaultHandler` turns off internal entity expansion
   ([09 §9.3](09-behavioral-spec.md#93-events)).
7. **Attribute normalization** that depends on ATTLIST types seen so far;
   default attributes are appended after specified ones; `xmlns` attributes
   are removed in namespace mode ([09 §9.4](09-behavioral-spec.md#94-text-normalization-rules)).
8. **Entity rules**: first declaration wins, general entity references in
   entity values are not expanded, "declarations may be missing" downgrades
   errors to skipped entities, declarations after an unread PE are ignored
   ([05](05-dtd-and-entities.md), [09 §9.6](09-behavioral-spec.md#96-entity-and-dtd-rules-in-detail)).
9. **Child parsers** for external entities: a copied DTD (general) or a
   shared DTD (parameter), the namespace context string, and limits charged
   to the root ([05 §5.5](05-dtd-and-entities.md#55-external-entities-and-child-parsers)).
10. **ABI details**: `userData` is the first struct field, error enum order,
    the `XML_Content` single-block layout freeable with `free()`, and the
    exact `XML_ErrorString` texts ([10](10-api-reference.md)).

## One-paragraph summary

Expat is a **streaming, push-based, non-validating** XML 1.0 parser with
callbacks in the style of SAX. The application pushes bytes in with
`XML_Parse`/`XML_ParseBuffer`, and Expat calls handlers as it recognizes
constructs. Internally there are three layers:

1. A **tokenizer** (`xmltok*`) that works on raw bytes in the document's
   encoding (UTF-8, UTF-16LE/BE, Latin-1, ASCII, or a user-defined 8-bit map).
   It returns one token at a time and never allocates.
2. A **prolog role machine** (`xmlrole.c`) that turns prolog/DTD tokens into
   semantic *roles*.
3. The **parser** (`xmlparse.c`), which owns all state. Through a
   function-pointer *processor* it decides which tokenizer mode to run, turns
   text into the API character type (`XML_Char`), keeps the DTD tables and
   element stack, expands entities, handles namespaces, and calls user
   handlers.

Recent releases also add a lot of defensive machinery: amplification limits
on entity expansion and on heap use, a salted SipHash for every hash table,
recursive algorithms rewritten as iterative ones, and overflow checks on
essentially all size arithmetic.

## Quick glossary

| Term | Meaning |
|------|---------|
| **Token** (`XML_TOK_*`) | Lexical unit returned by the tokenizer, for example `XML_TOK_START_TAG_WITH_ATTS` |
| **Role** (`XML_ROLE_*`) | Meaning of a prolog token in its grammatical context, for example `XML_ROLE_ATTRIBUTE_NAME` |
| **Processor** | `Processor *m_processor`: the function that will consume the next chunk of input (`prologProcessor`, `contentProcessor`, …) |
| **Encoding** (`ENCODING`) | Vtable of scanners and helpers specialized for one byte encoding |
| **Internal encoding** | The encoding Expat itself uses for stored text (UTF-8, or UTF-16 with `XML_UNICODE`). Entity replacement text is tokenized again in this encoding |
| **`XML_Char`** | The character type of the public API: `char` (UTF-8) by default, or 16-bit / `wchar_t` with `XML_UNICODE` |
| **Root parser** | The parser the application created. **Child parsers** are made by `XML_ExternalEntityParserCreate` for external entities and point back through `m_parentParser` |
| **GE / PE** | General entity (`&name;`) / parameter entity (`%name;`) |
| **Direct / indirect bytes** | Input bytes from the application vs. bytes produced by entity expansion; used for amplification accounting |
