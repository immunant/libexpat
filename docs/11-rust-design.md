# 11. Design proposal: a safe Rust reimplementation

This document proposes a design for an idiomatic, safe Rust reimplementation
of Expat that is a **drop-in replacement** for the C library: same `expat.h`,
same ABI, same observable behavior ([09](09-behavioral-spec.md),
[10](10-api-reference.md)). It uses the standard library and crates from
[blessed.rs](https://blessed.rs) where possible.

There are two parts:

* **Part A** (11.1–11.7) is the target design, described as a clean rewrite.
* **Part B** (11.8–11.12) describes how the plan changes when the starting
  point is the output of [c2rust](https://github.com/immunant/c2rust)
  instead. The destination is the same; the route is different.

---

# Part A: target design

## 11.1 Constraints that drive the design

| Constraint | Source | Design consequence |
|---|---|---|
| Stateless tokenizer that rescans partial tokens; any split of the input gives the same events | [02 §2.1](02-tokenizer.md#21-the-scanner-contract), [09 §9.2](09-behavioral-spec.md#92-lifecycle-and-state-machine) | Scanners are pure functions `(&[u8]) -> Scan`. Positions are offsets, not pointers. |
| Suspend and resume at any token, including inside entity expansion | [04 §4.7](04-parser-core.md#47-parsing-status-suspend-resume-abort), [05 §5.4](05-dtd-and-entities.md#54-expanding-internal-entities-without-recursion) | All parse state lives in the parser struct. Nothing resumable sits on the Rust stack, so there are no async or coroutine tricks. |
| No recursion driven by input | [07 §7.5](07-memory-and-security.md#75-removing-recursion) | Keep the processor-enum plus re-enter loop, and the explicit entity stacks. Content models are built flat. |
| Handlers re-enter the API with the same `XML_Parser` (`XML_StopParser`, `Set*Handler`, `DefaultCurrent`, position queries, `ExternalEntityParserCreate`) | [04](04-parser-core.md), [10 §10.4](10-api-reference.md#104-handler-registration) | This is the main safety problem. It is solved with a split-borrow `Context` in the core and a Cell-based header in the FFI layer (11.3.5, 11.4.2). |
| A PE child shares the parent's DTD; a GE child gets a deep copy; limits are charged to the root | [05 §5.5](05-dtd-and-entities.md#55-external-entities-and-child-parsers) | The parent lends its DTD to a PE child for the length of the callback. GE children use `Dtd: Clone`. The root's shared counters are atomics. |
| User memory suite, untracked buffers, `XML_Content` freeable with `free()`, alloc-failure tests | [07 §7.1](07-memory-and-security.md#71-pluggable-allocation-and-the-allocation-tracker), [09 §9.9](09-behavioral-spec.md#99-memory-and-limits) | Allocator-generic, fallible collections (11.3.8). |
| ABI details: `userData` at offset 0, error enum order, exact error strings, build variants | [10](10-api-reference.md) | The FFI crate treats upstream `expat.h` as the source of truth and verifies layouts (11.4). |

## 11.2 Workspace layout

```
expat-rs/
├── expat-tok/     #![no_std] + alloc, #![forbid(unsafe_code)]
│                  tokenizer, encodings, name tables, prolog role machine
├── expat/         #![no_std] + alloc (std feature, default on), #![forbid(unsafe_code)]
│                  parser core, DTD, entities, namespaces, accounting, safe API
├── expat-capi/    crate-type = ["cdylib", "staticlib"]; ALL unsafe code lives here
│                  libexpat.so.1 / libexpat.a / expat.dll, upstream expat.h installed verbatim
├── expat-conformance/  links upstream runtests.c + xmlwf against expat-capi (cc crate)
└── fuzz/          cargo-fuzz: split-invariance + differential vs. C expat
```

The two pure crates use `forbid(unsafe_code)`. The only unsafe code is in
`expat-capi`, which keeps the audit surface small. `no_std` + `alloc` serves
embedded users. The `std` feature adds the `EXPAT_*_DEBUG` environment
variables and the `std::error::Error` impls.

**Crates:**

* **std**, plus these from blessed.rs:
  * `bitflags` for handler interests and flags.
  * `thiserror` for errors.
  * `getrandom` for the hash salt. It covers getrandom, arc4random,
    `/dev/urandom` and BCrypt, which replaces the hand-written entropy ladder.
  * `memchr` for fast data-run scanning.
  * Dev only: `proptest`, `criterion`, `libfuzzer-sys`, `bindgen`, `cc`.
* **Two crates outside blessed.rs**, both needed so allocations can go
  through the C memory suite on stable Rust:
  * `hashbrown`, which is std's own map implementation, used here with an
    allocator parameter.
  * `allocator-api2`, its companion crate.
* **SipHash-2-4** is about 60 lines written in-crate, or the `siphasher`
  crate. std's `RandomState` cannot take an explicit salt, which
  `XML_SetHashSalt` and child-parser salt sharing both need.

## 11.3 Core design (`expat-tok` + `expat`)

### 11.3.1 Text and code units

* **All text is valid Unicode after decoding.** Expat rejects surrogates,
  overlong UTF-8, U+FFFE/U+FFFF, and `&#0;`. So the core can hand out `&str`.
  Because XML text can never contain U+0000, every stored string can also
  carry a hidden trailing NUL. The FFI layer then gets C strings with no
  copy: `XStr<'a>` wraps a slice that includes the NUL and exposes
  `.as_str()` (without the NUL) and `.with_nul()`.
* **The `XML_UNICODE` builds** need UTF-16 as the internal encoding. That is
  observable, because entity text is re-tokenized in the internal encoding,
  which changes the byte counts in the accounting tests. The C preprocessor
  template becomes monomorphization:

  ```rust
  pub trait CodeUnit: sealed::Sealed + Copy + Eq + 'static {
      type Str: ?Sized;                      // str  | [u16]
      type InternalEnc: Encoding;            // Utf8Internal | Utf16NativeInternal
  }
  pub type Parser<'r, H> = GenericParser<'r, u8, H, Global>;   // what Rust users see
  ```

  The safe API defaults to `u8`. The generic is `#[doc(hidden)]` and exists
  for the wide FFI build.

### 11.3.2 Tokenizer

`xmltok_impl.c` included three times becomes a generic over an encoding
trait. The `_ns` twins become a const generic. Encodings are dispatched
through an enum, not a vtable:

```rust
pub trait Encoding {
    const MINBPC: usize;                                   // 1 or 2
    fn byte_type<const NS: bool>(&self, s: &[u8]) -> ByteType;
    fn is_name_char(&self, s: &[u8]) -> bool;  /* nmstrt, invalid, to_utf8, to_utf16, update_position… */
}
pub enum Scan { Tok(Token, usize), MaybeIncomplete(Token, usize) /* the "-tok" case */,
                Partial, PartialChar, None, Invalid(usize), TrailingCr, TrailingRsqb }

pub fn content_tok<E: Encoding, const NS: bool>(enc: &E, s: &[u8]) -> Scan;  // prolog_tok, cdata_tok, ignore_tok,
                                                                            // attribute_value_tok, entity_value_tok, get_atts…
pub enum Enc { Detecting(InitState), Utf8, Latin1, Ascii, Utf16Le, Utf16Be,
               Unknown(Box<UnknownEncoding, A>), Internal }
```

* **`Token`, `ByteType` and `Role` are real Rust enums.** The order-dependent
  `ATTRIBUTE_TYPE_CDATA + i` becomes a method.
* **Byte-type tables are `const` arrays** built by `const fn`. The
  `#define BT_COLON BT_NMSTRT` trick becomes `if NS`.
* **`nametab.h` is ported once** as a generated `static` table, with a check
  that it matches the C bitmaps byte for byte.
* **Detection (`initScan`) is an explicit `Enc::Detecting` state.** C patches
  `*encPtr` in place to switch encodings; this replaces that. Once detected,
  the parser replaces `Enc`.
* **Unknown encodings** use
  `trait UnknownEncodingSource { fn map(&self) -> &[i32; 256]; fn convert(&self, b: &[u8]) -> i32; }`.
  Dropping it calls the C `release` callback (in the FFI layer).
* **The "validate once, extract fast" `getAtts` second pass is kept.** Bounds
  checks are written as slice patterns and iterator walks. Benchmark against
  C with `criterion` from day one.

### 11.3.3 Prolog role machine

The function pointer that serves as state becomes an enum. The `m_decl*`
register fields become a typed enum, so invalid combinations cannot be
represented:

```rust
pub struct PrologMachine { state: PState, level: u32, include_level: u32,
                           document_entity: bool, in_entity_value: bool }
enum PState { Prolog0, Prolog1, Prolog2, Doctype0, /* …5 */, InternalSubset,
              ExternalSubset0, ExternalSubset1, Entity0, /* …10 */, Attlist0, /* …9 */,
              Element0, /* …7 */, Notation0, /* …4 */, CondSect0, /* …2 */,
              DeclClose { role_none: Role }, Error }
impl PrologMachine { pub fn token_role(&mut self, tok: Token, text: &[u8], enc: &Enc) -> Role }
```

The transition table in
[03 §3.5](03-prolog-state-machine.md#35-appendix-complete-transition-table)
becomes a table-driven unit test, one row per test case.

### 11.3.4 Parser state and the driver loop

The processor function pointer becomes
`enum Processor { PrologInit, Prolog, Content, CdataSection, IgnoreSection, Epilog, InternalEntity, ExtEntityInit(u8), ExtEntityContent, ExtParEntInit, ExtParEnt, EntityValueInit, EntityValue, Error(ErrorCode) }`.
`call_processor` stays a `loop { match self.processor { … } }` with the
`reenter` flag and the reparse-deferral check, exactly as in
[04 §4.6](04-parser-core.md#46-callprocessor-the-driver-loop). Errors stay
sticky through `Processor::Error`.

* **The input buffer holds offsets, not pointers.** It is `Vec<u8, A>` with
  `start` (keeping `XML_CONTEXT_BYTES` of context), `ptr` and `end` offsets.
  The event span is
  `Span { src: Source::Document | Source::Entity(depth), range }`. Buffer
  moves no longer invalidate anything, so `storeRawNames` and the pointer
  fixups disappear.
* **Tag stack:** `Vec<Tag>`, where each tag holds `raw: Vec<u8>` and
  `name: Vec<C>` (the expanded name, NUL-terminated). The stack is not
  truncated on pop. A separate depth counter is used, so popped buffers are
  reused like `m_freeTagList`. Each tag stores its own expanded name, so the
  end-tag rebuild step goes away: C had to rebuild because children
  overwrite the shared buffer.
* **Open entities:** three `Vec<OpenEntity>` stacks (internal, attribute,
  value), each holding
  `{ entity: EntityId, pos: usize, start_tag_level, between_decl, event: Range<usize> }`.
  `processed` and `has_more` move out of the DTD into this record. The `open`
  flag stays on the entity in the DTD, so PE children that share the DTD
  still see it for recursion detection.

### 11.3.5 Events, handlers, and the borrowing model

This is the main design decision. Handler arguments borrow the input buffer,
the tag stack and the scratch space, while the handler needs to change
parser state. The parser struct is therefore split into disjoint field
groups, and event emission happens only where the borrows can be split field
by field:

```rust
struct GenericParser<'r, C, H, A> {
    input: InputBuffer<A>,      // shared-borrowed during callbacks (zero-copy char data)
    tags: TagStack<C, A>,       // shared (element names)
    scratch: Scratch<C, A>,     // shared (attribute values, converted text, PI/comment data)
    dtd: DtdSlot<'r, C, A>,     // Owned(Box<Dtd>) | Lent(&'r mut Dtd)
    ctl: Control,               // &mut during callbacks: status, position cache, event span, base, flags
    root: RootRef<'r>,          // &'r RootShared (atomics) or owned
    handler: H, /* tokenizer state, ns, … */
}

pub struct Context<'a, C> { ctl: &'a mut Control, input: &'a InputBuffer,
                            dtd: DtdAccess<'a, C>, root: &'a RootShared, /* … */ }
impl Context<'_, C> {
    pub fn stop(&mut self, resumable: bool) -> Result<(), ErrorCode>;
    pub fn position(&mut self) -> Position;             // lazy line/col update lives in ctl
    pub fn byte_index(&self) -> Option<u64>;  pub fn byte_count(&self) -> usize;
    pub fn default_current(&mut self, h: &mut impl Handler<C>);  // uses its own scratch buffer
    pub fn input_context(&self) -> Option<(&[u8], usize)>;
    pub fn specified_attribute_count(&self) -> usize;  pub fn id_attribute_index(&self) -> Option<usize>;
    pub fn set_base(&mut self, b: Option<&C::Str>);    pub fn set_default_expand(&mut self, on: bool);
    // valid only inside external_entity_ref:
    pub fn external_entity_parser<H2>(&mut self, ctx: &C::Str, enc: Option<&str>, h: H2)
        -> Result<GenericParser<'_, C, H2, A>, ErrorCode>;   // deep-copies the DTD
    pub fn param_entity_parser<H2>(&mut self, enc: Option<&str>, h: H2)
        -> Result<GenericParser<'_, C, H2, A>, ErrorCode>;   // LENDS &mut Dtd; lifetime-bound to the callback
}
```

Handlers return `Handled`, which is how the default-handler fallback works:

```rust
pub trait Handler<C: CodeUnit = u8> {
    fn interests(&self) -> Interests { Interests::all() }   // bitflags; perf/alloc hint only
    fn start_element(&mut self, cx: &mut Context<C>, name: XStr<C>, atts: Attributes<C>) -> Handled { Handled::No }
    fn character_data(&mut self, cx: &mut Context<C>, text: &C::Str) -> Handled { Handled::No }
    fn default(&mut self, cx: &mut Context<C>, raw: &C::Str) {}
    fn external_entity_ref(&mut self, cx: &mut Context<C>, r: ExternalRef<C>) -> ExtOutcome { ExtOutcome::NoHandler }
    fn unknown_encoding(&mut self, name: &C::Str) -> Option<Box<dyn UnknownEncodingSource>> { None }
    fn not_standalone(&mut self, cx: &mut Context<C>) -> bool { true }
    // … one method per expat handler; ~25 total
}
```

* `Handled::No` means "no specific handler", so the core falls back to
  `default()` with the raw text. This reproduces the fallback rule in
  [09 §9.3](09-behavioral-spec.md#93-events) without tracking whether a
  method was overridden.
* `interests()` only gates work the C library skips when a handler is
  missing: building content models, storing doctype strings, computing
  default text. The FFI layer reports interests exactly, so allocation counts
  match C.
* `XML_SetDefaultHandler` turning off entity expansion is a separate flag,
  `set_default_expand`, not a presence check.

**Rule enforced by code structure:** never hold a DTD borrow across a DTD
mutation or a lending callback.

* Entity text is re-sliced by index for each token
  (`&dtd.text(id)[pos..]`). `storeAtts` can then intern element types and
  attribute IDs while internal entity text is being tokenized.
* The `ExternalEntityRef` arguments (base, system ID, public ID, context) are
  copied into scratch first, so the DTD can be lent during that call.

### 11.3.6 DTD and entities

* **Tables are allocator-aware `hashbrown::HashTable<u32>` indexes over
  arenas.** For example `Vec<Entity>` addressed by typed IDs: `EntityId`,
  `ElementTypeId`, `AttrId`, `PrefixId`. Names live in an append-only string
  arena as spans. This replaces the intrusive `NAMED` struct layout, the
  casts, and the "entry->name == name means new" check. Insertion-order
  iteration makes `getContext()` deterministic, which is a harmless
  improvement over C (11.7).
* **`dtdCopy` becomes `#[derive(Clone)]`.** The hasher state, and with it the
  salt, is cloned too, which is the reason C needed to share the salt. The
  scaffold is not copied: it is only live inside an `<!ELEMENT>`.
* **`prefix->binding` moves out of the DTD.** It becomes
  `ns.current: Vec<Option<BindingId>>`, indexed by `PrefixId`, so the DTD
  only holds declarations.
* **The `name[-1]` flag byte** becomes a per-parser `Vec<u32>` of generation
  stamps indexed by `AttrId`. "Seen on this tag" means
  `stamp[id] == tag_gen`, so nothing has to be cleared. `m_nsAtts` uses the
  same versioning.
* **Content models:** the scaffold is already a flat, index-linked array.
  `build_model` emits a flat BFS `Vec<ContentNode>` with contiguous children.
  The safe API exposes `ContentModel<'a>`, a view over that slice. The FFI
  layer copies it into one C block in a single loop, without recursion
  (11.4.4).
* **Declaration registers** become a `DeclState` enum
  (`Entity { id: Option<EntityId>, … }`, `Attlist { elem, att, ty, … }`, …).
  Making it an enum enforces "first declaration wins": an ignored declaration
  is just `id: None`.

### 11.3.7 Namespaces and attributes

* **Bindings** are a `Vec<Binding>` arena with a free list. Each binding has
  a `prev_for_prefix` link and a `next_on_tag` link, both
  `Option<BindingId>`. These are the same two lists threaded through one set
  of nodes as in
  [06 §6.4](06-data-structures.md#64-namespace-bindings-prefix-binding), but
  with indices instead of pointers.
* **`store_atts` follows the eight steps in
  [04 §4.9](04-parser-core.md#49-storeatts-attributes) in order.** It builds
  attribute values into `scratch.atts` (NUL-terminated) plus a
  `Vec<(Span, Span)>`. `Attributes<'a>` is an iterator view over them. It
  must preserve: specified attributes first, defaulted ones after; `xmlns`
  attributes removed in namespace mode; the specified count and ID index;
  duplicate detection by QName and then by expanded name.

### 11.3.8 Memory: fallible, allocator-generic, tracked

* **The core is generic over `A: allocator_api2::alloc::Allocator + Clone`,
  defaulting to `Global`.** All collections come from `allocator-api2` or
  `hashbrown`. This is needed because the alloc tests (`alloc_tests.c`,
  `nsalloc_tests.c`) inject a failure at every allocation through the user's
  memory suite.
* **Two allocators are threaded through.** `Tracked<A>` charges the root's
  `AllocTracker` before forwarding; it is used by the DTD, stacks and
  scratch. Plain `A` is used for the input buffer and content models, which C
  deliberately leaves untracked
  ([07 §7.1](07-memory-and-security.md#71-pluggable-allocation-and-the-allocation-tracker)).
* **Every growth is fallible**: `try_reserve` followed by `push`, with `?`
  mapping to `ErrorCode::NoMemory`. This is enforced with
  `clippy::disallowed_methods` in `clippy.toml`, which bans `Vec::push`,
  `extend`, `String::push_str` and similar in the core. Checked arithmetic
  (`checked_add`/`checked_mul`, also mapping to `NoMemory`) replaces the
  `INT_MAX - y` idioms.
* **Panics:** the core is written so it does not panic.
  `clippy::indexing_slicing`, `arithmetic_side_effects` and `unwrap_used` are
  denied in the core. The FFI layer still adds `catch_unwind` as a backstop.

### 11.3.9 Security mechanisms

| Mechanism | Rust form |
|---|---|
| Amplification accounting ([07 §7.2](07-memory-and-security.md#72-billion-laughs-protection-amplification-accounting)) | `RootShared { direct, indirect: AtomicU64, limits }`. `accounting_diff_tolerated()` is called at exactly the C call sites, including the deliberate `XML_ACCOUNT_NONE` cases, so the acc-test counters match. |
| Alloc tracker ([07 §7.1](07-memory-and-security.md#71-pluggable-allocation-and-the-allocation-tracker)) | `Tracked<A>`. Same thresholds and formulas. Byte totals differ from C (11.7). |
| Hash flooding ([07 §7.3](07-memory-and-security.md#73-hash-flooding-protection)) | Salted SipHash-2-4 `BuildHasher` on every table, with the salt stored in `RootShared`. `getrandom` supplies the salt, created on the first parse. |
| Reparse deferral ([07 §7.4](07-memory-and-security.md#74-reparse-deferral)) | Same heuristic in `call_processor`. The default is a `static AtomicBool`, which tests can write to. |
| No recursion ([07 §7.5](07-memory-and-security.md#75-removing-recursion)) | Processor loop and entity stacks. Content models are built flat. |
| Threading ([07 §7.7](07-memory-and-security.md#77-threading)) | Atomics for the shared root counters, so GE children are `Send` and honor C's "separate thread" claim. A PE child is lifetime-bound to its callback. |

### 11.3.10 Safe public API (sketch)

```rust
let mut p = Parser::builder()
    .namespaces(Some('|')).return_triplets(false)
    .param_entity_parsing(PeParsing::UnlessStandalone)
    .amplification_limit(100.0, 8 << 20)
    .build(MyHandler::default())?;
p.feed(chunk)?;                  // -> Result<Status /* Ok | Suspended */, Error>
let buf = p.buffer(4096)?;  /* fill */  p.parse_buffer(n, false)?;
p.finish()?;                     // is_final = true
p.resume()?;  p.reset(None)?;  p.handler_mut();  p.into_handler();
```

`Error { code: ErrorCode, position: Position }`, where `ErrorCode` is a
`#[non_exhaustive]` enum with `thiserror` messages identical to
`XML_ErrorString`.

## 11.4 FFI layer (`expat-capi`)

### 11.4.1 ABI contract

* **Upstream `expat.h` and `expat_external.h` are installed unchanged.** The
  Rust declarations are hand-written. A CI test uses `bindgen` on the real
  headers to check every struct size, alignment and field offset, and every
  enum value: `XML_Content`, `XML_Encoding`, `XML_Feature`,
  `XML_Memory_Handling_Suite`, `XML_ParsingStatus`, `XML_AttrInfo`.
* **C enums and bools come in as integers.** C can pass any integer value,
  and an out-of-range enum discriminant is undefined behavior in Rust. So
  `XML_ErrorString(c_int)`, `XML_SetParamEntityParsing(c_int)` and
  `XML_Bool` = `c_uchar` all validate explicitly.
  `SetReparseDeferralEnabled` must reject anything other than 0 or 1.
* **Types:** `XML_Index`/`XML_Size` are `c_long`/`c_ulong`, or `i64`/`u64`
  under the `large-size` feature. Watch Windows, where `long` is 32-bit.
* **Static data:** error strings and the version string are `c"…"` literals.
  `XML_GetFeatureList` returns a `static` array wrapped in a `Sync` newtype.
* **Linking and packaging:**
  * `build.rs` emits `cargo:rustc-cdylib-link-arg` for
    `-Wl,--version-script=libexpat.map` and `-soname,libexpat.so.1`, and uses
    a `.def` file on Windows.
  * `cargo-c` builds `expat.pc`.
  * A small CMake package config keeps `find_package(expat)` users working.

### 11.4.2 Parser object and re-entrancy

```rust
#[repr(C)]
pub struct XML_ParserStruct {
    user_data: Cell<*mut c_void>,          // offset 0 — XML_GetUserData() reads it directly
    handler_arg: Cell<*mut c_void>,
    handlers: CHandlers,                   // Cell<Option<extern "C-unwind" fn(..)>> × ~25
    active: Cell<Option<NonNull<Context<'static, Ch>>>>,  // set only while a C handler runs
    in_api_call: Cell<bool>,
    mem: XML_Memory_Handling_Suite,
    family: Family,                        // root shared state, live-children count, lent-DTD token
    core: UnsafeCell<CoreParser>,          // GenericParser<'static, Ch, CBridge, SuiteAlloc>
}
```

* **The struct is allocated with the user's `malloc_fcn`**, initialized with
  `ptr::write`, and torn down with `drop_in_place` followed by `free_fcn`.
* **`CBridge` implements `Handler`** by reading the `Cell`s in the header.
  `XML_Set*Handler` only writes a `Cell`, so it is safe from inside a
  callback and takes effect at the next event.
* **Before calling into C**, the bridge stores the `Context` pointer in
  `active` (lifetime erased) and restores the previous value afterwards.
  Re-entrant API calls (`XML_StopParser`, `GetCurrentLineNumber`,
  `DefaultCurrent`, `GetSpecifiedAttributeCount`, `GetInputContext`,
  `Get/SetBase`, `ExternalEntityParserCreate`, …) go through `active` when it
  is set, and touch `core` directly only when it is not. No `&mut` to the
  core is ever created while one is already live.
* **Misuse that is undefined behavior in C becomes a defined error.**
  Examples are `XML_Parse`, `XML_ResumeParser`, `XML_ParserReset` or
  `XML_ParserFree` on a parser that is in the middle of a callback. These
  return `XML_STATUS_ERROR` / `XML_ERROR_UNEXPECTED_STATE`, or defer the free
  until the call returns.

### 11.4.3 Child parsers across the C boundary

* **GE child:** owns a cloned DTD. It holds a pointer to the root's
  `RootShared`, which sits in its own heap allocation so its address is
  stable. The root's `Family` counts live children. If
  `XML_ParserFree(root)` runs while children are alive, the root becomes a
  zombie that is actually freed with its last child. In C, this order of
  frees is a use-after-free.
* **PE child:** receives a pointer to the parent's lent DTD. When the
  `ExternalEntityRef` call returns, the parent revokes the lend. A PE child
  the application failed to free inside the handler (a contract violation) is
  poisoned. Its later calls fail with `XML_ERROR_UNEXPECTED_STATE` instead of
  touching freed memory.

### 11.4.4 Remaining FFI details

* **Attributes for the start handler:** a reused `Vec<*const XML_Char>`
  pointing at the core's NUL-terminated scratch strings, with a terminating
  null pointer.
* **`XML_Content`:** one `malloc_fcn` block of `[XML_Content × n][names]`,
  filled in a single pass over the flat BFS model. `XML_FreeContentModel`
  calls `free_fcn` on it.
* **`XML_GetBuffer`:** returns `vec.as_mut_ptr().add(end)`. No Rust
  reference to the buffer exists until `XML_ParseBuffer` runs.
* **Unwinding:** handler typedefs and exports use `extern "C-unwind"`, so C++
  exceptions thrown from handlers propagate the same way they do through C
  expat. `catch_unwind` around every export turns a Rust panic into a sticky
  `XML_ERROR_UNEXPECTED_STATE`, so a panic never crosses into C. Handlers
  that `longjmp` out are documented as unsupported.
* **Build variants as Cargo features of `expat-capi`:**
  * Additive features: `unicode`, `unicode-wchar` (Windows), `large-size`,
    `attr-info`.
  * `dtd` and `ge` are default-on core features. Turning them off produces
    the no-DTD and `XML_GE=0` variants.
  * `context-bytes-0`.
  * `testing` exports the internal hooks: `testingAccountingGetCountBytes*`,
    `g_reparseDeferralEnabledDefault`, `g_bytesScanned`,
    `expat_malloc/realloc/free`,
    `_INTERNAL_trim_to_complete_utf8_characters`, `unsignedCharToPrintable`.
  * `XML_NS` is always on, and `XML_MIN_SIZE` is a no-op.

## 11.5 Verification

Test oracles, in order of value:

1. **Upstream `runtests`** (about 12k lines) built with `cc`, unchanged, and
   linked against `expat-capi` with the `testing` feature. This is the
   definition of "drop-in".
2. **The W3C suite** via upstream `xmlwf`, diffed against
   `xmltest.log.expected`.
3. **Differential fuzzing** of C expat (built with `cc`) against the Rust
   version on the same bytes and random chunk splits. It compares normalized
   event logs, error code and position.
4. **`proptest` split-invariance:** events must be identical for any split,
   modulo how character data is chunked.
5. **Table tests** generated from
   [02 §2.10](02-tokenizer.md#210-appendix-lexical-rules-a-reimplementation-must-match)
   and [03 §3.5](03-prolog-state-machine.md#35-appendix-complete-transition-table).
6. **Miri** on the core and on Rust-driven FFI tests, mainly to check the
   `active`/`UnsafeCell` re-entrancy design.

## 11.6 Phased plan (clean rewrite)

1. **`expat-tok`.** Optionally, also expose it through a temporary shim that
   implements `xmltok.h`/`xmlrole.h`'s C interface and link it into C
   `xmlparse.c`. That runs the whole upstream test suite against the Rust
   tokenizer early, and the shim is thrown away afterwards.
2. **Core content path:** buffers, processors, tags, attributes, namespaces,
   safe API, differential fuzzing.
3. **DTD and entities:** entity stacks, child parsers and lending,
   accounting, salt.
4. **`expat-capi`:** `runtests`, xmltest, the variant matrix, packaging.
5. **Hardening:** pass the alloc-failure tests, Miri, performance parity
   with C (target within about 10–20%), port `xmlwf`.

## 11.7 Planned deviations and risks

**Deviations:**

* **`getContext()` prefix order** becomes insertion order (C: hash order).
  It is documented as unspecified, and `setContext` accepts any order.
* **Alloc-tracker byte totals** differ because the data structures differ.
  Defaults and formulas are identical. Watch tests that pin exact
  thresholds.
* **Undefined-behavior misuse becomes defined errors**: re-entrant
  `XML_Parse`, freeing a parent before its children, keeping a PE child
  after the handler returns.
* **`XML_DefaultCurrent` inside a character-data handler** no longer
  overwrites the handler's argument buffer. C does overwrite it (shared
  `m_dataBuf`). Callers cannot rely on either behavior.

**Risks:**

* **Exact character-data chunking.** Splits come from one call per token
  plus the 1024-unit conversion buffer. Keep `DATA_BUF_SIZE` and the
  per-token emission identical, because tests compare the pieces.
* **Exact error positions for the ⚑ cases** in
  [09 §9.7](09-behavioral-spec.md#97-position-and-error-reporting).
* **Allocation-site fidelity.** Allocations happen in different places than
  in C, so alloc tests that assert a specific failure count may need review.
  Tests that loop over "fail the nth allocation" should pass unchanged.
* **Tokenizer bounds checks.** Benchmark early. The fallback is an audited,
  `#[cfg]`-gated `unsafe` fast path in a single module. It must not be the
  default.
* **`XML_UNICODE_WCHAR_T` on platforms where `wchar_t` is 32 bits.** Match
  upstream and reject it at build time.

---

# Part B: starting from c2rust output

Starting from c2rust changes the route, not the destination. The target
architecture of Part A stays the same. What changes is how you get there.
Instead of writing a new library and then chasing compatibility, you start
with a library that is already compatible and safe it up one step at a time,
re-running the full test suite after every step.

## 11.8 What you get from c2rust on day one

| Concern | Clean rewrite | c2rust start |
|---|---|---|
| ABI compatibility | Built and verified at the end (phase 4) | Already there: `#[repr(C)]` structs, `#[no_mangle] extern "C"` exports, `userData` at offset 0 |
| Upstream `runtests`, xmltest, fuzz corpora | Usable only once the FFI layer exists | Usable from the first commit, as a regression gate |
| ⚑ quirks, char-data chunking, error positions, accounting call sites, allocation call sites | Re-derived from the docs and fixed through test failures | Kept automatically, because it is the same algorithm |
| Shippable artifact | Late | Immediately, though it is all `unsafe` |

The main risk moves. With a rewrite, the risk is behavioral fidelity. With
c2rust, the risk is that you never finish removing the C shape and end up
with "C in Rust syntax".

## 11.9 Revised phases

**Phase 0: baseline.**

* Get the translation building as a `cdylib`.
* Link it into upstream `runtests` and `xmlwf`, and make xmltest, fuzz-corpus
  replay and differential fuzzing against C pass.
* Record a `criterion` performance baseline.
* Translate with `XML_TESTING` defined, then put those hooks behind a
  `testing` Cargo feature by hand. Otherwise `runtests` cannot link.

**Phase 1: platform and libc cleanup (mechanical).**

* Replace `memcpy`, `memmove`, `memcmp`, `strlen` and the like with slice
  operations wherever the lengths are known.
* Replace the entropy `#ifdef` ladder, which c2rust froze for the
  translation host, with `getrandom`.
* Use `std::env` for the `EXPAT_*_DEBUG` variables.
* Use static `c"…"` strings for `XML_ErrorString`.
* Clean up relooper artifacts (`current_block` loops) in small functions.
  Leave `doProlog`, `doContent` and `storeAtts` for later.

**Phase 2: replace the leaf modules rather than refactoring them.**

* The tokenizer, role machine, SipHash and name tables have narrow
  interfaces, and c2rust is a poor starting point for them. It emits the
  `normal_`/`little2_`/`big2_` × NS template copies as separate functions,
  roughly six near-identical copies of the tokenizer, and the role machine is
  still function-pointer state.
* Write `expat-tok` from the docs and C. Plug it in behind the existing
  `ENCODING` vtable using thin wrappers: build a `&[u8]` from `(ptr, end)`,
  call the generic scanner, write `*nextTokPtr`.
* The translated scanners become an **in-process differential oracle**. Fuzz
  old and new scanners on the same bytes, then delete the old ones.

**Phase 3: data structures, one at a time.**

* Order: `HASH_TABLE` → `hashbrown` with typed IDs; `STRING_POOL` → arenas
  with spans; `TAG`/`BINDING` free lists → `Vec`-backed stacks;
  `OPEN_INTERNAL_ENTITY` lists → three `Vec` stacks; the `name[-1]` flag →
  generation stamps.
* Each replacement stays behind the old function names (`lookup`,
  `poolStoreString`, …) until every caller has moved.
* Allocation: put an `allocator-api2` `SuiteAlloc`/`Tracked<A>` on top of
  the existing `m_mem` suite and tracker logic. The size header in
  `expat_malloc` becomes unnecessary because `deallocate` receives the
  `Layout`. The alloc tests keep passing the whole time.

**Phase 4: pointers to offsets (cross-cutting, the hardest step).**

* `m_bufferPtr`, `m_eventPtr`, `m_positionPtr`, `rawName`,
  `ATTRIBUTE.valuePtr` and the entity `internalEventPtr`s are used
  everywhere.
* Add offsets alongside the pointers, move consumers over one at a time,
  then delete the pointers. `storeRawNames` and the URI-buffer fixups go away
  at the end.
* Turn `m_processor` into a `Processor` enum and the `m_decl*` registers
  into a `DeclState` enum.
* Split `doProlog` into one method per role group, using
  [05 §5.3](05-dtd-and-entities.md#53-processing-declarations-roles--actions)
  as the map.

**Phase 5: the borrowing split, then extract the crates.**

* Split the parser struct into field groups (`input`, `tags`, `scratch`,
  `dtd`, `ctl`).
* Introduce `Context` and the `Handler` trait.
* The old exported functions become the `expat-capi` layer: a Cell-based
  header plus `CBridge`.
* Only now do `expat-tok` and `expat` become separate crates with
  `forbid(unsafe_code)`.

**Phase 6: variants and hardening.** Add the `CodeUnit` generic for the
`XML_UNICODE` builds, add the `dtd`/`ge` features back, run Miri, and reach
performance parity.

## 11.10 Rewrite or refactor, per component

| Component | Recommendation | Why |
|---|---|---|
| `xmltok*` (tokenizer, transcoders, detection) | **Rewrite** | Template duplication; a narrow, pure interface; the translation is useful as an oracle |
| `xmlrole.c` | **Rewrite** | Small; turning function-pointer state into an enum touches every line anyway; [03 §3.5](03-prolog-state-machine.md#35-appendix-complete-transition-table) gives the full table |
| `siphash.h`, `nametab.h`, byte tables | **Rewrite** or regenerate | Trivial |
| Hash table, string pool, free lists | **Replace** behind the old call signatures | std/`hashbrown` equivalents exist |
| `xmlparse.c` processors, `doContent`, `storeAtts`, entities, accounting | **Refactor in place** | This is where the quirks live; c2rust's fidelity is worth the most here |
| Public API functions | **Refactor** into `expat-capi` | Already ABI-correct |

## 11.11 Hazards specific to the c2rust route

1. **Converting to references too early creates undefined behavior that the
   tests won't catch.** Translated functions take `parser: XML_Parser` (a raw
   `*mut`), and C handlers call back in with the same pointer. While
   everything is raw pointers, that aliasing is as legal as it is in C. Once
   a function that can reach a handler holds `&mut XML_ParserStruct`, any
   re-entrant call violates Rust's aliasing rules. The code still "works", so
   `runtests` stays green.
   * **Rule:** anything on a path that calls a handler keeps raw-pointer
     style until phase 5 lands the header/`Context` split.
   * Leaf helpers that never call handlers can move to `&mut` freely.
   * Run Miri with Tree Borrows on Rust-driven re-entrancy tests to catch
     mistakes.
2. **The build configuration is frozen.** c2rust evaluates `#if`, so
   `XML_DTD`, `XML_GE`, `XML_UNICODE`, `XML_CONTEXT_BYTES`, `XML_ATTR_INFO`
   and `XML_LARGE_SIZE` are fixed by the translation. Translate the default
   configuration and add the variants back by hand during the refactor. Ship
   UTF-8 first; the wide builds wait for the `CodeUnit` generic. Translating
   each variant separately and diffing the outputs is not worth it.
3. **The platform is frozen too.** `c_long` sizes, `BYTEORDER`, and
   `#ifdef _WIN32` branches all reflect the host the translation ran on.
   Windows and 32-bit `long` support come back only as modules are
   rewritten, so put a Windows CI job in early to keep this honest.
4. **Relooper control flow** in the large `switch`/`goto` functions is the
   most laborious hand cleanup. Do it together with the per-role split in
   phase 4, not as a separate pass.
5. **The C architecture can harden into the final design.** Intrusive lists,
   processor function pointers and NUL-terminated pools are easy to keep
   "because it works". To prevent that:
   * Add `#![deny(unsafe_code)]` per module as each one becomes safe.
   * Keep an unsafe-count ratchet in CI (`cargo geiger`, or counting
     `unsafe` blocks) so the number only goes down.
6. **Performance drift.** The baseline runs at roughly C speed, and
   replacing pointers with bounds-checked slices can slow down hot loops.
   Gate every step against the phase-0 benchmark.

## 11.12 What stays the same, and recommendation

**Unchanged from Part A:**

* The target crate layout and the borrowing model (`Context`, `Handler`
  returning `Handled`, lent vs. cloned DTD).
* The FFI header design: `Cell`-based handler slots, the `active` context
  pointer, C enums received as integers, zombie or poisoned children,
  `C-unwind`.
* Every security mechanism and default limit.
* The test oracles: upstream suites first, then differential fuzzing
  against C.
* The list of deliberate deviations (11.7).

**Recommendation:** use a hybrid. Rewrite the leaves (tokenizer, role
machine, containers) from the docs, with the c2rust code as the in-process
oracle. Refactor `xmlparse` incrementally from the translation.

Compared with a clean rewrite, this keeps a compatible, shippable library at
every commit, and keeps the fidelity risk confined to the code you rewrite.
The cost is a longer middle stretch: phases 4–5 are cross-cutting and depend
on the raw-pointer rule in hazard 1 of 11.11.
