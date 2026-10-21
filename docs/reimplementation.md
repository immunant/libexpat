# Reimplementation plan and verification

This chapter turns the source walkthrough into an implementation sequence.
It is a suggested design, not a claim that the C implementation has these
physical module boundaries. Use the [architecture map](architecture.md) to
find the corresponding code and the other chapters for semantic requirements.

## Choose and record the compatibility target

Decide explicitly whether the replacement promises compatible logical XML events,
the full source-level Expat API, or binary replacement for a particular build.
The latter adds exact public type layouts, calling conventions, symbol exports,
macro behavior, feature reporting, and allocator interoperability. Record the
reference commit and build options; there is no single behavior configuration
covering all Expat feature builds.

For an initial full-feature reference, use the checkout's default UTF-8 API,
GE/DTD/NS enabled, and context retention. Then test zero context, reduced-feature
builds, optional attribute positions, and wide/large-position ABIs separately.
If a stage intentionally lacks DTDs, namespaces, or a particular encoding,
identify that stage as a subset rather than silently approximating the feature.

The exact character-class/name sets and declaration state transitions should
come from the checked-in sources or an independently verified equivalent.
Typical 'XML-like' regular expressions do not cover the reference grammar.
Similarly, accepting all Unicode identifiers or validating every DTD constraint
would change the target behavior.

## Suggested implementation boundaries

| Component | Inputs/outputs | Persistent state |
| --- | --- | --- |
| ABI/API facade | Public calls and callback registration | Callback arguments, options, lifecycle, error reporting |
| Input cursor | Byte spans and commit requests | Unconsumed suffix, context, absolute offset basis |
| Decoder/scanner | Bounded bytes and lexical mode -> token/range/need-more/error | Initial encoding detection and custom decoder lifecycle |
| Prolog grammar | Token and spelling -> grammatical role | Grammar state, group/include depth, document/external context |
| Semantic processor | Role/content token -> state changes/events | Current phase and declaration-under-construction |
| Declaration store | Intern names, defaults, entity text/IDs | Document lifetime tables and storage |
| Namespace scope | Prefix declarations and QNames -> expanded names | Binding stacks and per-tag unwind list |
| Entity scheduler | Replacement source and usage category | Explicit frames, recursion guard, progress, depth boundaries |
| Resource manager | Consumed spans and allocation deltas | Shared root budgets, secret hash key, diagnostic counters |
| Event/location layer | Semantic event and originating byte range | Current event, lazy source position, length/ownership conventions |

In a memory-safe implementation, use owned strings/arenas, explicit byte offsets,
checked integer conversions, and clear borrow scopes. Interned IDs can replace
the scratch character before a C string; immutable expanded names can replace
mutable binding-buffer suffixes. These are implementation choices. Preserve
normalization timing, callback ordering, child sharing, and error behavior.

## Implementation stages and exit criteria

1. **Public skeleton and checked input buffering.** Implement constructors,
   custom allocation family, free/reset, statuses, error storage, buffer commit,
   and preserved incomplete suffixes. Exercise zero-length input/finalization,
   allocation failures, and invalid public call sequences before parsing markup.

2. **Encoding/scanner foundation.** Implement bounded UTF-8 scanning, XML character
   checks, lexical modes and partial-result contract. Add UTF-16 endian variants,
   ASCII/Latin-1, auto-detection, declaration switching, and unknown encodings.
   Every token/character/BOM boundary must work split across input calls.

3. **Document/content processor.** Add prolog/content/epilog transitions, tag
   stack and exact matching, empty elements, text normalization, numeric and
   predefined references, comments, PIs, and streaming CDATA. Check extra roots,
   text outside the root, missing root, malformed names, and unfinished final input.

4. **Callbacks, locations, and continuation.** Add full event lifetimes, default
   fallback and forwarding, per-event original positions, retained context,
   handler mutation, stop/resume/abort, and preservation of names across buffer
   moves. Run every supported stop boundary with final and nonfinal input.

5. **Declaration grammar and storage.** Add DOCTYPE, ELEMENT, ATTLIST, ENTITY,
   NOTATION, public IDs, defaults, non-CDATA normalization, ID metadata, and
   optional owned content models. Keep syntax checks active without handlers;
   preserve nonvalidating behavior.

6. **Entity scheduler and external context.** Add iterative expansion separately
   for content/prolog, attributes, and replacement-value construction. Add
   declaration origin/completeness/standalone rules, PE policies, conditional
   sections, external fragment parsers, shared external PE DTDs, and foreign DTDs.
   Resolve resources through test fixtures/application callbacks.

7. **Namespace layer.** Implement QName checks, all declarations before prefix
   expansion, defaulted bindings, scope shadowing/undeclaration, expanded duplicate
   attributes, triplets, reserved bindings, inherited context, and namespace
   callback mutation/stop interactions.

8. **Hardened execution.** Root accounting and checked allocation should already
   surround relevant stages; now verify all charge sites, exemptions, failure
   codes, hash hardening, large-token deferral, deep nesting, model flattening,
   and correct reset/free behavior. Do not defer safety checks until after
   running hostile corpora.

9. **Compatibility closure.** Check every exported function, configuration,
   error family, and callback combination. Compare public struct ABI/export lists
   if required. Use both the reference regression corpus and differential
   mutation/fuzzing; address performance and memory differences separately from
   event equivalence.

Each stage needs successful and failing tests. A few well-formed example documents
cannot establish compatibility for any completed subsystem.

## Core invariants

- Consumed source bytes, stored replacement code units, replacement progress
  bytes, callback lengths, and absolute indices have explicit, distinct units.
- Incomplete syntax remains available across calls, and no committed event is
  emitted twice when an incomplete suffix is retried.
- Final input bypasses deferral and either completes the permitted grammar or
  returns a definite error; it cannot remain indefinitely 'need more'.
- Raw open-tag names survive buffer replacement; namespace name storage survives
  child scope changes; converted outputs remain valid throughout their callbacks.
- Scope exit restores every prefix exactly once, with element/namespace callback
  order preserved, including empty elements and stop requests.
- A general replacement cannot cross its caller's element boundary; a PE cannot
  close the DOCTYPE or leave forbidden partial grammar.
- An entity remains open until its descendants close, including a descendant
  scheduled by its final reference token; every expansion context uses this guard.
- Attribute defaulting/type normalization is keyed by the current lexical element
  and attribute declaration. Expanded duplicate identity excludes the prefix.
- Callbacks see the correct argument convention and source event; source byte
  indices are never manufactured from normalized/converted output lengths.
- All descendants use root limits and secret state. Shared DTDs have one owner
  and synchronous mutation discipline; child teardown precedes ancestor teardown.
- Realloc failure preserves the old object; partial construction can always be
  freed; decoder release runs once; raw and tracked allocation families match.
- Reset removes document-specific meaning while preserving only the options,
  registrations, budgets, and caches the implementation actually retains.

## Differential harness

Build the reference and replacement with the same configuration. Run them in
separate processes or through a controlled symbol boundary. For each input and
API script, record status/error, callback type/payload, callback-time locations,
attribute metadata, external resolution calls, ownership/free actions, and
state after each public call. Use a fixed hash salt in deterministic tests.

Use two comparison levels:

1. **Logical events:** combine adjacent text callbacks only when their event
   kind and intervening semantic boundaries permit it. Preserve CDATA/element/DTD
   boundaries, attribute order, namespace events, literal-reference spelling
   in default output, and all nontext payloads.
2. **API execution:** preserve per-call statuses, suspension points, locations,
   setter effects, and finalization. This catches bugs hidden by a coalesced
   event comparison. Character/default chunk sizes are not generally a public
   guarantee, so classify harmless partition differences explicitly; do not
   erase differences in stop behavior or source-position contracts.

For every small fixture, try one-shot input, every single split point, all
one-byte chunks, selected multiple splits, and deterministic random partitions.
Also vary `XML_GetBuffer` reservation sizes independently from committed byte
counts; the deferral heuristic uses the application's request size. Repeat with
final input in its last data chunk and with a separate zero-length final call.
Overwrite/reuse caller buffers after each `XML_Parse` return to expose retained
borrowed pointers. Do not make invalid API use part of an 'equivalence' test
unless that misuse has a defined contract.

External entity tests should use an in-memory map of system IDs to byte strings,
log resolution arguments, and create general versus PE children with the proper
context. Include independent encodings and malformed external fragments. No
network/file resolver is needed for deterministic parser tests.

## Behavioral test matrix

| Area | Essential fixtures and interactions |
| --- | --- |
| Document structure | Empty/whitespace/comment-only input, one root, multiple roots, prolog/epilog junk, unclosed/mismatched tags, empty elements, deep nesting |
| Lexical boundaries | Every delimiter split in comment, PI, start/end tag, quoted value, numeric/named reference, CDATA opener/closer, DOCTYPE, conditional section |
| Characters | Illegal controls, NUL, bad UTF-8, overlong/surrogate/out-of-range sequences, incomplete final characters, UTF-16 pairing and byte order, non-ASCII name tables |
| Encoding setup | Each built-in, split BOMs, protocol override, declaration mismatch, unknown handler rejection/acceptance/malformed maps/release and reset |
| Text normalization | LF/CR/CRLF in every context, numeric whitespace references, internal CR preservation, text/default differences, CDATA data, supplementary character output |
| Attributes | Missing separators, bad quotes/literal `<`, lexical duplicates, declared types, collapse across entities, defaults, required/fixed nonvalidation, ID index, optional original offsets |
| DTD grammar | All declaration kinds, optional ID forms, enumerations/notations, group quantifiers, invalid mixed connectors, declaration duplicates, INCLUDE/IGNORE nesting, public-ID validity |
| Entities | Nested/empty/forward/cyclic entities, delayed interpretation, recursion in all three expansion contexts, balanced markup, unparsed references, illegal external attribute references |
| DTD completeness | PE NEVER/UNLESS_STANDALONE/ALWAYS, missing/skipped declarations, standalone yes/no/absent, PE declaration origins, forbidden internal-subset PE locations |
| External input | General fragment empty/multiple roots/text, optional text declaration, copied context, inherited handlers/options, synchronous shared PE state, foreign DTD without DOCTYPE |
| Namespaces | Prefix declared later/defaulted, default versus attribute naming, nested shadowing, empty default URI, illegal prefix undeclaration, reserved bindings, expanded duplicates, triplets/separators |
| Callbacks | Each handler absent/present, default versus expand-default, DefaultCurrent, namespace callback changes element handler, declaration precedence, stored model lifetime |
| Control | Stop in empty start/end, namespace starts/ends, text conversion and nested entities; resume final remainder, repeat suspension, abort suspended parser, reject PE suspension |
| Errors/positions | All enum families, callback-time source offsets, final truncation positions, entity-origin location, unknown/incorrect encoding location, zero-context behavior |
| Memory | Fail successive allocations/reallocations; child creation/reset/free after partial construction; pool relocation and caches; decoder release; content model ownership |
| Limits | Threshold just below/at/above, finite/infinite/NaN factors, zero direct bytes, external descendant aggregation, tracked/raw allocation exemptions, reset settings |
| Performance | Increasing unfinished names/comments/attributes/PIs, variable buffer requests, deeply nested entities/groups/elements, many attributes/names, hash collisions, enabled/disabled deferral |

## Error coverage

Use exact enum values and error strings from `expat.h`/`XML_ErrorString`. The
following groups ensure the API/error matrix is not limited to ordinary XML
syntax failures:

| Family | Error names after `XML_ERROR_` |
| --- | --- |
| Basic parse | `SYNTAX`, `NO_ELEMENTS`, `INVALID_TOKEN`, `UNCLOSED_TOKEN`, `PARTIAL_CHAR`, `TAG_MISMATCH`, `DUPLICATE_ATTRIBUTE`, `JUNK_AFTER_DOC_ELEMENT`, `UNCLOSED_CDATA_SECTION` |
| Declaration/encoding | `MISPLACED_XML_PI`, `UNKNOWN_ENCODING`, `INCORRECT_ENCODING`, `XML_DECL`, `TEXT_DECL`, `PUBLICID` |
| Entity/DTD | `PARAM_ENTITY_REF`, `UNDEFINED_ENTITY`, `RECURSIVE_ENTITY_REF`, `ASYNC_ENTITY`, `BAD_CHAR_REF`, `BINARY_ENTITY_REF`, `ATTRIBUTE_EXTERNAL_ENTITY_REF`, `EXTERNAL_ENTITY_HANDLING`, `NOT_STANDALONE`, `ENTITY_DECLARED_IN_PE`, `INCOMPLETE_PE` |
| Namespace | `UNBOUND_PREFIX`, `UNDECLARING_PREFIX`, `RESERVED_PREFIX_XML`, `RESERVED_PREFIX_XMLNS`, `RESERVED_NAMESPACE_URI` |
| API/lifecycle | `FEATURE_REQUIRES_XML_DTD`, `CANT_CHANGE_FEATURE_ONCE_PARSING`, `SUSPENDED`, `NOT_SUSPENDED`, `ABORTED`, `FINISHED`, `SUSPEND_PE`, `INVALID_ARGUMENT`, `NO_BUFFER`, `NOT_STARTED` |
| Resource/internal | `NO_MEMORY`, `AMPLIFICATION_LIMIT_BREACH`, `UNEXPECTED_STATE` |

`NONE` is the successful no-error value. `UNEXPECTED_STATE` is chiefly an internal
invariant backstop; do not invent a malformed XML fixture merely to make it
reachable. Some API functions return errors without poisoning parsing, while a
fatal processor error becomes sticky. Verify return value, stored error, public
lifecycle, and subsequent call behavior as separate observations.

## Existing test and fuzz infrastructure

| Source | Use |
| --- | --- |
| [`basic_tests.c`](../expat/tests/basic_tests.c) | Parsing/encoding/DTD/control regressions, deep entities, large-token complexity, reparse options |
| [`ns_tests.c`](../expat/tests/ns_tests.c) | Namespace expansion, duplicate/reserved names, buffer reuse, separator behavior, callback mutation |
| [`alloc_tests.c`](../expat/tests/alloc_tests.c) | Allocation failure paths and allocation tracker behavior |
| [`nsalloc_tests.c`](../expat/tests/nsalloc_tests.c) | Namespace allocation failure handling |
| [`acc_tests.c`](../expat/tests/acc_tests.c) | Direct/indirect accounting precision, protection API, isolated external parsers |
| [`misc_tests.c`](../expat/tests/misc_tests.c) | Auxiliary API and implementation regression cases |
| [`common.c`](../expat/tests/common.c), [`handlers.c`](../expat/tests/handlers.c) | Input helpers, fixtures, callback recorders |
| [`runtests.c`](../expat/tests/runtests.c) | Suite assembly; repeats with chunk sizes 0 through 5 and deferral disabled/enabled |
| [`xmltest.sh`](../expat/tests/xmltest.sh) and [`xmltest.log.expected`](../expat/tests/xmltest.log.expected) | Separate XML corpus/conformance workflow; check corpus prerequisites |
| [`xml_parse_fuzzer.c`](../expat/fuzz/xml_parse_fuzzer.c) | Parse API, stop requests, reset, namespace/external parser use |
| [`xml_parsebuffer_fuzzer.c`](../expat/fuzz/xml_parsebuffer_fuzzer.c) | Buffer API counterpart |
| [`xml_lpm_fuzzer.cpp`](../expat/fuzz/xml_lpm_fuzzer.cpp), [`xml_lpm_fuzzer.proto`](../expat/fuzz/xml_lpm_fuzzer.proto) | Structured XML mutation |
| [`benchmark/benchmark.c`](../expat/tests/benchmark/benchmark.c) | Existing throughput benchmark; supplement with adversarial work-count tests |

Useful named regressions include `test_big_tokens_scale_linearly`,
`test_deep_nested_entity`, `test_deep_nested_attribute_entity`,
`test_deep_nested_entity_delayed_interpretation`, `test_nested_entity_suspend`,
`test_reparse_deferral_is_inherited`, `test_bypass_heuristic_when_close_to_bufsize`,
`test_ns_tagname_overwrite`, `test_ns_duplicate_attrs_diff_prefixes`,
`test_start_ns_clears_start_element`, `test_ns_separator_in_uri`,
`test_alloc_tracker_getbuffer_unlimited`, and
`test_alloc_reset_after_external_entity_parser_create_fail`.

The test library uses `XML_TESTING` internals. Tests that directly inspect C
implementation counters/allocators need a replacement adapter or an equivalent
test at its own internal boundary; they cannot all be linked unchanged solely
against a public ABI implementation. C++ companion files exercise the source
under C++ compilation as well; they are not separate XML semantics.

## Reference build commands

Run from repository root, with an available C/C++ toolchain and CMake:

```sh
cmake -S expat -B /tmp/expat-reference-build \
  -DEXPAT_BUILD_TESTS=ON \
  -DEXPAT_BUILD_DOCS=OFF \
  -DEXPAT_BUILD_EXAMPLES=OFF \
  -DEXPAT_BUILD_TOOLS=OFF
cmake --build /tmp/expat-reference-build --parallel
ctest --test-dir /tmp/expat-reference-build --output-on-failure
```

Use separate build directories for a matrix, for example
`-DEXPAT_CONTEXT_BYTES=0`, `-DEXPAT_MIN_SIZE=ON`, `-DEXPAT_ATTR_INFO=ON`,
`-DEXPAT_LARGE_SIZE=ON`, `-DEXPAT_DTD=OFF`, and the legal combination
`-DEXPAT_DTD=OFF -DEXPAT_GE=OFF`. Also test namespace support disabled and runtime
nonnamespace parsing. Wide ABI variants need the supported toolchain/target and
possibly separate public-API fixtures; see [build compatibility](api-and-build.md).

Run memory/undefined-behavior instrumentation and fuzzing with the corresponding
toolchain/configuration documented in the repository. Keep logical correctness,
resource accounting, memory safety, and performance regressions as separate
results. Passing a conformance corpus does not imply stop/resume or ABI correctness;
passing public API fixtures does not establish resistance to hostile expansion.
