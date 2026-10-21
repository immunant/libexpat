# Safe Rust migration from c2rust output

Refactor the c2rust translation incrementally into safe Rust while preserving
Expat's grammar, tables, and observable behavior. Keep a working parser at each
stage. The endpoint has safe parsing modules backed by small, audited FFI and
allocation layers, with a C API that can replace the corresponding Expat build.

This plan targets the C implementation described in the [code map](README.md).
It assumes the starting implementation is generated Rust; the first milestone
establishes its equivalence to C before substantive refactoring. The existing
[reimplementation guide](reimplementation.md) supplies the behavioral invariants
and verification matrix that apply throughout the migration.

## Compatibility baseline

Start with the documented default configuration: UTF-8 API, GE/DTD/NS enabled,
and 1,024 context bytes. Pin the source commit, generated configuration, target
ABI, c2rust revision, and Rust toolchain. Binary compatibility is a promise for
a particular build; additional configurations need their own verification.

C2rust processes preprocessed C, so one translation captures one platform and
feature configuration. Adding Cargo features afterward cannot recover branches
excluded during translation. Preserve translations or preprocessed sources for
additional configurations, then reconcile them into explicit Rust configuration
boundaries. See the [C2Rust platform FAQ](https://github.com/immunant/c2rust#faq).

Translate the three library translation units: `xmlparse.c`, `xmltok.c`, and
`xmlrole.c`. The scanner templates included by `xmltok.c` must not become
independent compilation units. See the [source inventory](architecture.md).

Maintain three test subjects:

- The original C library as the independent reference.
- An immutable, minimally repaired c2rust translation.
- The actively refactored Rust implementation.

First establish that the translation matches C, then compare subsequent changes
against both. Compilation alone establishes neither equivalence nor memory
safety. C2rust describes its output as an unsafe starting point requiring further
migration work. See the [C2Rust migration model](https://github.com/immunant/c2rust#intro).

## Target architecture

Extract these responsibilities gradually from the translation rather than
rewriting them simultaneously:

| Component | Rust representation | Existing implementation to preserve initially |
| --- | --- | --- |
| Input | Owned buffer, committed length, offsets, retained context | `XML_Parse`, `XML_GetBuffer`, `XML_ParseBuffer` |
| Scanner | Bounded byte slices, lexical modes, typed outcomes | `xmltok` scanners and exact character tables |
| DTD grammar | Explicit grammar state and role enum | `xmlrole` transition functions |
| Semantic engine | Separate lifecycle, processor phase, and continuations | `callProcessor`, `doContent`, `doProlog` |
| Declaration storage | Typed IDs into document-owned storage | DTD records, pools, and hash tables |
| Namespace scopes | Binding records, shadow links, per-element unwind lists | `storeAtts`, `addBinding` |
| Entity execution | Explicit frame stacks for all three expansion contexts | Existing iterative entity machinery |
| Callback dispatch | Prepared payloads and explicit return continuations | Individual callback sites |
| Resources | Per-parser allocator suite and shared root budgets | Raw/tracked allocation families and accounting |

Keep public lifecycle, processor phase, lexical context, and DTD grammar as
independent state machines. Combining them into one large enum would obscure
valid combinations such as suspension during internal-entity processing.

Use `#[forbid(unsafe_code)]` for completed algorithm modules. Initially the
generated implementation remains an explicitly unsafe subsystem; its scope
shrinks as ownership boundaries become enforceable. A safe wrapper around
arbitrary unchecked pointers is not a completed conversion.

## Callback dispatch and borrowing

A mechanical conversion from `*mut XML_ParserStruct` to `&mut Parser` is unsound
if that borrow spans an application callback that accesses the same state.
Callbacks can change handlers, query locations, request suspension, and create
external parsers that access shared state.

The eventual dispatch protocol is:

1. Borrow engine state briefly to prepare the next callback and its continuation.
2. Publish the callback-visible event and location.
3. End engine and shared-DTD borrows.
4. Invoke application code with payload storage valid throughout the callback.
5. Reacquire state and continue according to the precise dispatch rules.

Dispatch remains synchronous and resumable at individual callback sites. A queue
of all events from a token would hide observable mutations: a namespace callback
can replace the element handler before that element's start event. Conversely,
some conversion paths cache handlers, so registrations must be reread according
to the existing dispatch site rather than a universal rule.

A stop request cannot simply prevent every subsequent callback. An empty-element
start callback can request suspension and still be followed by its paired end
callback. These details need explicit continuations. See the
[callback and suspension behavior](parsing.md).

`UnsafeCell` may support the handle's internal representation, but it does not
permit overlapping mutable references. A `RefCell` guard or mutex held across
application code has the same structural problem. See
[Rust aliasing requirements](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#aliasing-rules).

## Allocation and failure handling

Ordinary `Vec`, `String`, `Box`, and `HashMap` allocations do not use the suite
passed to `XML_ParserCreate_MM`. The standard allocator-parameterized `Vec` API
is experimental, and `Vec::from_raw_parts` cannot adopt arbitrary suite
allocations. See the [allocator API](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new_in)
and [raw allocation contracts](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts).

Retain the translated allocation machinery first, then expose safe, fallible
storage abstractions over it: a vector-like buffer, typed record storage, and
stable string blocks. Their implementation contains audited unsafe code; parser
algorithms operate through safe methods and slices. Standard collections remain
useful in test tooling and wherever their allocator behavior fits the contract.

Preserve the two allocation families:

| Family | Allocations |
| --- | --- |
| Tracked suite | Parser internals, declarations, workspaces, cached storage |
| Raw suite | Input buffer, transferred `XML_Content` models, public memory helpers |

Growth must report failure without losing the old allocation. Avoid hidden
infallible allocation in formatting, collection insertion, reference counting,
and cloning. A process-global allocator switch is unsuitable because different
parsers can use different suites. Catching a panic cannot turn an aborting
allocation failure into `XML_ERROR_NO_MEMORY`.

Changed representations can change memory consumption and allocation counts.
Preserve the public budget rules, shared-root accounting, and failure behavior;
separately track differences in private allocation sequences and threshold
crossings. See the [allocation requirements](security-and-performance.md).

## Storage and pointer migration

Replace pointer relationships in complete ownership groups:

| Translated representation | Replacement |
| --- | --- |
| Input pointers and pointer subtraction | Source identity plus checked byte ranges |
| Persistent declaration pointers | `EntityId`, `ElementId`, `AttributeId`, `PrefixId` |
| Scratch byte before an interned attribute name | Per-start-tag marks indexed by attribute ID |
| Tag and entity linked lists | Explicit stacks or indexed frames |
| Mutable URI buffers with tag-pointer repair | Stored URI/local/prefix components and callback-time formatting |
| Scanner integer results and output pointers | Typed outcomes carrying only valid ranges |
| Processor and grammar function pointers | Explicit states, converted independently |

Introduce distinct types for byte offsets, API code-unit counts, and absolute
indices. Entity replacement length is measured in `XML_Char` units while resumed
progress is measured in bytes.

Offsets need a source identity: document input, entity replacement text, and
external input are different sources. They also need a lifetime rule. An offset
into a buffer becomes invalid after the referenced bytes are discarded, even
though it avoids a dangling pointer.

During migration, keep stable pool allocations wherever generated code retains
addresses. Converting pool blocks to a growable vector while legacy pointers
still escape would reintroduce invalidation. Move each connected group of owners
and consumers together. See the [storage and ownership map](data-structures.md).

For scanning, preserve the distinctions between partial token, partial character,
trailing CR, trailing brackets, provisional prolog token, invalid input, and end
of available input. Some outcomes do not initialize `next`; the safe interface
must not manufacture a range from that pointer.

Retain the exact naming tables, encoding detection, and separate internal and
external character classification. A generic Unicode identifier test or
whole-input transcoding step would change behavior. See the
[scanner contracts](tokenization.md).

## External parsers and entity execution

External general-entity children copy declaration state and restore context.
External parameter-entity children share the DTD and execute synchronously. All
descendants use root budgets and the root hash secret.

Represent these as distinct relationships. Shared DTD access must use short
borrows that end before callbacks, allowing a parameter-entity child to mutate
declarations safely. The Rust-native API can encode child lifetime and synchronous
use through scoped operations; the C facade retains Expat's lifetime contracts.

Preserve iterative execution for content, attribute values, and entity-value
construction. An entity remains open until its descendants finish, including a
descendant scheduled by its final token. Also keep teardown iterative so deeply
nested input cannot cause recursive Rust destruction to exhaust the stack. See
the [entity relationships and continuations](dtd-and-entities.md).

## C API and binary compatibility

Initially retain generated public types and entry points. Once dispatch and
ownership are separated, move exports into a dedicated facade with:

- Exact C types, enum values, layouts, calling conventions, and nullable function
  pointers.
- A stable parser address with user data at offset zero: `XML_GetUserData`
  directly dereferences that field.
- Correct NUL termination, pointer-plus-length payloads, and UTF-8/UTF-16 variants.
- Raw-suite allocation for transferred content models, preserving contiguous
  child arrays.
- Matching exports, ELF symbol versions, Windows definitions, library naming,
  and generated headers.
- Fallible error handling without Rust unwinding through C entry points.

For `XML_GetBuffer`, expose writable storage through the unsafe boundary and
treat only committed bytes as initialized input. Do not construct an initialized
Rust slice over unwritten capacity.

Retain the original public headers as the compatibility authority. Bindings
generators can assist verification, but regenerated headers should not silently
replace Expat's macros and configuration behavior. See the
[ABI requirements](api-and-build.md).

Implement reset from an explicit field policy. Namespace settings survive;
unknown-encoding registration survives while its active decoder is released;
expansion settings reset; allocation-tracker settings and cached live allocations
persist. A generic `Default` replacement would be incorrect.

## Migration sequence

| Stage | Change | Exit criterion |
| --- | --- | --- |
| 1 | Freeze translation, toolchain, configuration, and C reference | Baseline API traces and regressions agree |
| 2 | Consolidate duplicated types; add units and checked conversions | No intended behavioral change |
| 3 | Convert `xmlrole`, then scanner helpers, into safe functions | Transition and split-input differential tests pass |
| 4 | Introduce allocator-aware storage and explicit callback continuations | Allocation failures, handler mutation, and stop/resume remain correct |
| 5 | Replace input/event pointers and storage ownership in connected groups | Buffer relocation, zero-context input, and callback locations agree |
| 6 | Convert declarations, attributes, namespaces, and entity frames | Defaults, ordering, shared DTDs, recursion checks, and deep nesting agree |
| 7 | Finish processor states and remove generated unsafe code | Parsing modules forbid unsafe; remaining unsafe is confined and audited |
| 8 | Complete configuration/platform coverage and optimize | ABI, security, and performance gates pass per configuration |

Preserve accounting charge sites, SipHash behavior, reparse deferral, tables, and
grammar while making ownership changes. Algorithmic optimization follows
demonstrated equivalence, so a failure has a small set of possible causes.

The first concrete milestone is a verified c2rust baseline with `xmlrole` and
scanner boundaries converted to safe Rust. That establishes useful safety
progress while retaining the mature semantic parser as the behavioral foundation.

## Standard library and dependency choices

For parser algorithms, favor slices, iterators, enums, `Option`, `Result`, checked
arithmetic, and standard UTF conversion primitives where their semantics match
Expat.

The useful [Blessed.rs](https://blessed.rs/crates) dependencies are modest:

| Dependency | Proposed use |
| --- | --- |
| `libc` | Default C allocator and platform interoperability |
| `bindgen` | Header-derived ABI verification bindings |
| `cc` | C probes and reference harness builds |
| `thiserror` | Optional Rust-facing error boilerplate |
| `rand` | OS-backed entropy for automatic salt initialization |
| `insta` | Reviewed trace snapshots |
| `criterion` | Performance comparisons |

Keep testing and build dependencies outside the runtime library where possible.
Retain the existing keyed hash initially. A later scanner optimization could use
[`memchr`](https://docs.rs/memchr/latest/memchr/), a justified dependency outside
that list.

## Verification gates

Use the existing regression and fuzz infrastructure, adapting tests that inspect
`XML_TESTING` internals rather than treating them as public ABI tests. Add
differential API scripts recording callbacks, statuses, errors, callback-time
locations, setter effects, external resolution, and ownership transfers.

The critical cases are:

- Every split point for small fixtures, one-byte chunks, random partitions, and
  separate zero-length final calls.
- Different buffer reservation and commit sizes, plus caller-buffer reuse after
  `XML_Parse`.
- Handler mutation, default reporting, `XML_DefaultCurrent`, repeated suspension,
  and paired callbacks.
- Successive allocation failures, failed child construction, decoder release,
  reset, and transferred model freeing.
- Shared DTD mutation, all entity expansion contexts, deep nesting, and budget
  boundaries.
- C layout/export probes and an already-linked consumer using the replacement
  library.
- Sanitizers and fuzzing for the FFI/storage layer; Miri for supported Rust-only
  components.

Preserve exact traces during the early refactors. If later optimizations change
permissible text partitioning, classify those differences explicitly without
coalescing away suspension or location failures. Expand beyond the initial build
to zero context, reduced features, attribute information, large positions,
wide-character APIs, and supported platforms. Use the
[existing verification plan](reimplementation.md) for the full matrix.
