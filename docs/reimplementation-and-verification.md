# Reimplementing and verifying Expat

[Guide index](README.md)

A reimplementation should first select its supported build/API profile, then establish a reference harness against this checkout. The sequence below separates suggested implementation choices from the existing architecture described in the other chapters. Internal representations may change; externally visible parsing, continuation, and ownership rules need explicit compatibility tests.

## Suggested module boundaries

| Module | State or contract to expose |
| --- | --- |
| Public API adapter | Expat-compatible functions/types, argument checks, feature selection, callbacks, and allocator interface |
| Input manager | Original byte storage, absolute offsets, context retention, pending ranges, final-input state |
| Encoding and lexical layer | Bounded scanners, partial/error distinctions, decoding/conversion, name tables, source positions |
| Prolog grammar | Explicit grammar state accepting tokens and yielding semantic roles |
| Semantic parser | Processor continuation, element stack, event dispatch, error state, suspension |
| DTD store | Interned identifiers, declarations, defaults, content models, declaration provenance |
| Namespace environment | Push/restore bindings, expanded-name formatting, duplicate identity |
| Entity executor | Separate content/attribute/value frames, cycle detection, context serialization, child creation |
| Resource state | Shared root accounting, allocation tracking, hashing policy, scan-work limits |

Use distinct types or clearly named fields for byte offsets, `XML_Char` lengths, Unicode scalars, and array-entry indexes. In a language with movable collections, stable IDs or arena handles can replace pointers. Store source spans as offsets when buffer relocation would invalidate references. A family-owned resource object can make the C implementation's implicit parent lifetime dependency explicit.

## Implementation sequence

1. **Freeze the target contract.** Record the reference commit, build flags, callback encoding, position widths, namespace mode, parameter-entity policy, and ABI goal. Keep unsupported features explicit rather than silently accepting a subset of their syntax.
2. **Implement bounded scanners and decoding.** Start with UTF-8 plus the complete/partial/invalid contract. Add split delimiters, XML name predicates, numeric-reference validation, line endings, and accurate source spans before building higher semantics.
3. **Implement document phases and core events.** Add prolog/content/epilog transitions, open-tag matching, empty elements, text, references, comments, PIs, and CDATA. Exercise whole-buffer and incremental feeds from the beginning.
4. **Add buffer and continuation behavior.** Support both input APIs, final zero-length calls, direct/buffered ownership, retained raw names, position queries, stop/resume, and deferred scans. Test stopping from callbacks while conversion still has buffered work.
5. **Add attributes and DTD state.** Normalize explicit values, retain element-specific type/default metadata, report declarations, and construct owned content models. Keep declaration values distinct from attribute values.
6. **Add namespaces.** Resolve after all declarations/defaults, restore scopes, validate reserved bindings, produce pair/triplet names, and detect expanded duplicates.
7. **Add internal entities using explicit frames.** Handle all three contexts, provenance, skipped declarations, recursive references, and entity-boundary well-formedness. Enforce accounting while implementing expansion, not after it is functional.
8. **Add external general and parameter entities.** Preserve the copy/share distinction, text declarations, inherited settings, context lifetime, family charging, and synchronous parameter-entity restrictions.
9. **Complete encoding and build profiles.** Add UTF-16 endianness/alignment, Latin-1, ASCII, unknown encodings, API UTF-16 output where targeted, DTD/GE/NS switches, attribute info, and context/large-size/min-size variants.
10. **Finish reuse, cleanup, and binary compatibility.** Exercise failure at every allocation site, reset after error/suspension, unknown-encoding release, child destruction, public struct layout, exported symbols, and calling conventions.

This order is an engineering plan, not a statement that an intermediate implementation is Expat-compatible. DTD, namespaces, and streaming semantics interact; preserve those interactions as each layer is added.

## Reference harness

Run the reference implementation and replacement through the same adapter and capture:

- All enabled callbacks, their payloads, attribute order, namespace boundaries, and declaration models.
- Parse/stop/resume status, error code, and location at chosen observation points.
- Specified-attribute count, ID index, byte counts, and optional attribute ranges during callbacks.
- External-reference arguments and application responses, including child-parser results.
- Memory-suite requests/releases and unknown-encoding release counts for failure tests.

Use two comparison modes. A semantic mode can coalesce adjacent character-data events within the same event context, since arbitrary callback segmentation is allowed. An exact mode must retain boundaries for suspension, default reporting, callback-time positions, and version-specific timing. Never coalesce across element, CDATA, namespace, declaration, or external-resource boundaries merely to make output match.

For malformed documents, compare the error code and source location as well as the callback prefix already delivered. Callback delivery is not transactional: a namespace callback can precede an error in the same tag, and a declaration callback can precede a later encoding or grammar error.

## Minimum behavior matrix

| Area | Cases to include |
| --- | --- |
| Document framing | Empty input, whitespace only, one root, two roots, missing end tag, mismatched name, trailing text, valid epilog comments/PIs |
| Chunk boundaries | Every split point of short inputs; byte-at-a-time feeds; randomized chunks; empty non-final calls; final empty call |
| Delimiters | Splits within `<?`, `?>`, `<!--`, `-->`, `<![CDATA[`, `]]>`, `/>`, references, and quoted attribute values |
| Encoding | Split BOMs, UTF-16 odd-byte chunks, invalid UTF-8, truncated multibyte characters, surrogate errors, declared/detected mismatch, unknown-encoding success/failure/release |
| Text and normalization | Literal CR/CRLF versus referenced CR/LF; attributes of CDATA and non-CDATA types; nested entities introducing whitespace; default-handler preservation |
| Names | Table boundary characters, illegal start characters, ordinary colons, namespace QNames with missing/multiple parts |
| Attributes | Raw duplicates, explicit before default, missing required attribute under non-validation, fixed-value override, ID index, many attributes, long values |
| Namespaces | Default inheritance/removal, prefix shadowing, implicit `xml`, reserved bindings, same URI under two prefixes, triplets, defaulted declarations, separator edge cases |
| DTD | All declaration kinds, public ID normalization, duplicate declarations, nested models, mixed content, group connector errors, INCLUDE/IGNORE nesting |
| Internal entities | Empty values, forward references used later, direct/indirect cycles, markup in replacement text, attributes rejecting external entities, unparsed references, cross-entity tags |
| External entities | Empty and multi-element fragments, text declarations, copied DTD, null context for parameter entities, missing handler, handler failure, deferred general entities |
| Parameter policy | NEVER/UNLESS_STANDALONE/ALWAYS, standalone declaration, skipped external subsets, foreign DTD with null identifiers, incomplete parameter declarations |
| Control | Suspend/abort from text/start/end/namespace/DTD callbacks, empty-element paired callbacks, repeated suspension, final-buffer resume, child/parent coordination, illegal API sequences |
| Reuse and failure | Reset after success/error/suspension, child reset rejection, custom allocator failures, realloc failure, unknown-encoding cleanup, model ownership |
| Resource behavior | Nested and repeated expansion, family accounting, allocation thresholds, scan work for huge unfinished tokens, deep models/entities, wide namespace-attribute sets |

## Concrete acceptance examples

These examples assume normal entity support and do not assert a particular segmentation of adjacent text callbacks.

| Input or operation | Expected result |
| --- | --- |
| `<r/>` | One start and one end event, then successful final completion |
| `<r>one&amp;two</r>` | Character data concatenates to `one&two` |
| `<r a="x&#10;y"/>` | Attribute contains LF between `x` and `y` |
| `<!DOCTYPE r [<!ATTLIST r a NMTOKENS #IMPLIED>]><r a="  x   y  "/>` | Value is `x y` |
| `<!DOCTYPE r [<!ATTLIST r a CDATA "v">]><r/>` | Default attribute `a="v"`; specified-attribute count is 0 |
| `<r a="1" a="2"/>` | `XML_ERROR_DUPLICATE_ATTRIBUTE` |
| `<r xmlns:p="urn:x" xmlns:q="urn:x" p:a="1" q:a="2"/>` in namespace mode | `XML_ERROR_DUPLICATE_ATTRIBUTE` despite distinct prefixes |
| `<r xmlns="urn:x" a="v"/>` with separator `|` | Element `urn:x|r`; attribute name remains `a` |
| `<r>&missing;</r>` with no DTD | `XML_ERROR_UNDEFINED_ENTITY` |
| `<!DOCTYPE r [<!ENTITY a "&a;">]><r>&a;</r>` | `XML_ERROR_RECURSIVE_ENTITY_REF` |
| `<r><![CDATA[x` with final input | `XML_ERROR_UNCLOSED_CDATA_SECTION` |
| `<r/><s/>` | `XML_ERROR_JUNK_AFTER_DOC_ELEMENT` |
| Resumable stop from start callback for `<r/>` | End callback may still occur before parse returns suspended; resume finishes without repeating either event |

The longer [execution walkthrough](parser-execution.md#a-complete-document-walkthrough) combines declarations, defaults, namespaces, numeric references, and an internal entity in one document.

## Existing regression and fuzzing sources

| Source | Coverage to reuse or study |
| --- | --- |
| [`basic_tests.c`](../expat/tests/basic_tests.c) | Encodings, text, DTDs, external entities, suspension, positions, buffer behavior, deferral |
| [`ns_tests.c`](../expat/tests/ns_tests.c) | Namespace expansion, binding errors, callbacks, duplicates |
| [`alloc_tests.c`](../expat/tests/alloc_tests.c) | Allocation-failure cleanup, tracked sizes/alignment, amplification and thresholds |
| [`nsalloc_tests.c`](../expat/tests/nsalloc_tests.c) | Allocation failures in namespace paths |
| [`acc_tests.c`](../expat/tests/acc_tests.c) | Direct/indirect byte accounting and expansion limits |
| [`misc_tests.c`](../expat/tests/misc_tests.c) | API and regression edge cases |
| [`handlers.c`](../expat/tests/handlers.c), [`common.c`](../expat/tests/common.c) | Callback assertions, parser setup, incremental-feed helpers, allocator instrumentation |
| [`runtests.c`](../expat/tests/runtests.c) | Suite assembly; repeats with chunk sizes 0 through 5 and reparse deferral off/on |
| [`xml_parse_fuzzer.c`](../expat/fuzz/xml_parse_fuzzer.c), [`xml_parsebuffer_fuzzer.c`](../expat/fuzz/xml_parsebuffer_fuzzer.c) | Byte-oriented entry points for both public input paths |
| [`xml_lpm_fuzzer.cpp`](../expat/fuzz/xml_lpm_fuzzer.cpp), [`xml_lpm_fuzzer.proto`](../expat/fuzz/xml_lpm_fuzzer.proto) | Structured XML fuzzing and its input model |
| [`xmltest.sh`](../expat/tests/xmltest.sh) | Integration with the separate XML conformance corpus; not automatically supplied by the two core CTest targets |
| [`benchmark/`](../expat/tests/benchmark/) | Existing throughput/parse benchmarking support |

Targeted starting points include `test_suspend_resume_internal_entity_issue_629`, `test_suspend_in_sole_empty_tag`, `test_set_reparse_deferral_on_the_fly`, `test_amplification_isolated_external_parser`, `test_alloc_tracker_getbuffer_unlimited`, and `test_alloc_tracker_pointer_alignment`. Tests that use private hooks may need adapters in a rewrite; retain the behavior being tested rather than duplicating C-private layouts unnecessarily.

## Build and run the C reference

From the repository root, with a C/C++ compiler and CMake available:

```sh
cmake -S expat -B /tmp/expat-docs-reference-build \
  -DEXPAT_BUILD_DOCS=OFF \
  -DEXPAT_BUILD_EXAMPLES=OFF \
  -DEXPAT_BUILD_TESTS=ON \
  -DEXPAT_SHARED_LIBS=OFF
cmake --build /tmp/expat-docs-reference-build --parallel 4
ctest --test-dir /tmp/expat-docs-reference-build --output-on-failure
```

The `runtests` and `runtests_cxx` targets cover C and C++ compilation paths. These commands build from this checkout and require no install step. Use a separate build directory for each feature combination and inspect the configure summary to verify the intended flags.

Beyond the baseline, exercise `EXPAT_CONTEXT_BYTES=0`, `EXPAT_ATTR_INFO=ON`, `EXPAT_LARGE_SIZE=ON`, `EXPAT_MIN_SIZE=ON`, `EXPAT_NS=OFF`, `EXPAT_DTD=OFF`, and the valid `EXPAT_DTD=OFF`/`EXPAT_GE=OFF` combination. Do not try DTD on with GE off. UTF-16 API variants need platform-aware harnesses: this CMake configuration rejects the bundled test suite for `EXPAT_CHAR_TYPE=ushort`, and `wchar_t` requires two-byte width and the documented toolchain/libc compatibility. These are build constraints, not evidence that UTF-16 input is unsupported in the ordinary UTF-8 API build.

For the replacement, run its own unit tests plus the differential harness under relevant memory/undefined-behavior instrumentation. Fuzz parser state operations as well as document bytes: stop/resume points, chunking, allocator failures, resets, unknown encoding descriptions, and external-handler responses are all part of the input space.

## Verification of this guide

For the commit identified in the [index](README.md), the baseline configure/build commands above completed successfully, and CTest passed both `runtests` and `runtests_cxx` (2 of 2 targets). A separate callback probe checked 14 cases covering the acceptance examples, the combined document walkthrough, and suspension from an empty-element start callback. Local documentation links and code-fence balance were also checked.

These checks validate the reference commands and illustrated behavior; they do not establish compatibility of a replacement implementation or validate all build variants. The internal-entity byte-count difference found by the callback probe is recorded in [API compatibility](api-compatibility.md#positions-and-byte-counts).

## Completion criteria

A replacement is ready for its declared profile when the reference event/error corpus matches under varied chunking, required APIs and feature variants work, every transferred/borrowed allocation obeys its lifetime, parser families preserve resource budgets, and large/deep malformed inputs remain bounded by the intended controls. Binary-replacement claims additionally require symbol/type/layout checks and real client linkage. Keep explicit records of deliberate differences, including stricter resource policy, altered callback timing, broader name acceptance, or unsupported build variants.
