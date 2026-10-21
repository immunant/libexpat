# Expat resource controls

[Guide index](README.md)

Resource behavior is part of this revision's parser design. [`xmlparse.c`](../expat/lib/xmlparse.c) contains expansion accounting, allocation tracking, hash protection, and partial-token reparse deferral; defaults live in [`internal.h`](../expat/lib/internal.h). These mechanisms address different costs and cannot replace one another.

## Entity expansion accounting

With `XML_GE=1`, `ACCOUNTING` tracks direct and indirect bytes at the root parser. `accountingDiffTolerated` receives a token and consumed source range plus one of three classifications:

| Classification | Meaning |
| --- | --- |
| `XML_ACCOUNT_DIRECT` | Bytes consumed from a parser's supplied input; counted as direct only for the root parser |
| `XML_ACCOUNT_ENTITY_EXPANSION` | Bytes processed as replacement text; counted as indirect |
| `XML_ACCOUNT_NONE` | Already accounted elsewhere; do not count again |

Bytes supplied to a child parser count as indirect even when its local operation is classified direct. Counting belongs to token consumption, not to the total number of bytes passed to `XML_Parse` or emitted through callbacks. Attribute and declaration paths use `NONE` in places to avoid charging ranges already covered by a larger token.

Let `D` be direct bytes and `I` indirect bytes. For nonzero `D`, the amplification factor is `(D + I) / D`. Parsing is allowed while total accounted bytes are below the activation threshold, or the factor is at most the configured maximum. The defaults are **8 MiB** (`8,388,608`) and **100.0**. Reaching the threshold does not by itself reject input; the ratio also matters.

When `D == 0`, `accountingGetCurrentAmplification` uses a 22-byte baseline, the length of the source's shortest external-entity declaration example, instead of dividing by zero. A faithful port needs this branch for isolated child activity. The byte counters and arithmetic require overflow handling; do not rely on wrapping arithmetic.

Partial/invalid/no-token results are filtered before calculating range lengths. Pending bytes are not charged again for every failed scan. Exceeding the expansion protection returns `XML_ERROR_AMPLIFICATION_LIMIT_BREACH` through the affected parsing path. The byte metric is an accounting measure of parser work, not a claim about serialized XML output size.

## Allocation amplification

`MALLOC_TRACKER` accounts for live internal allocations across a parser family. `expat_malloc` prepends a size field and alignment padding, checks the prospective root total, allocates through the configured suite, and updates the total. `expat_free` recovers the original pointer and size and subtracts it. `expat_realloc` checks increases before allocation and updates totals only after success; size zero uses the free path.

Once prospective tracked allocation reaches the **64 MiB** (`67,108,864`) threshold, the permitted ratio is:

```text
tracked live bytes / direct input bytes <= maximum allocation factor
```

The default factor is **100.0**. A zero direct-input count can produce an infinite ratio in the floating-point calculation. Limits are root-owned and include child-parser internal allocations. A policy denial is returned as an allocation failure, generally becoming `XML_ERROR_NO_MEMORY` or a null constructor result, not the entity-expansion error code.

The tracker deliberately excludes:

- The input buffer, leaving input-size limits to the application.
- Application allocations through `XML_MemMalloc`/`XML_MemRealloc`.
- Exported `XML_Content` models, which use the raw suite for compatibility with application freeing behavior.

Therefore this is not a total process-memory cap. Free-list allocations remain live and charged until actually freed, including across reset. The parser object has special construction/destruction handling because allocator and root state must exist before ordinary tracked allocation can work.

The public allocation and expansion setters reject child parsers and invalid factors. Both mechanisms depend on root state; a port using independent resource objects or reference counting should keep the same family-wide charging semantics.

## Avoiding repeated scans of large tokens

Scanners retry incomplete tokens from their beginning. Feeding an unfinished token one byte at a time and scanning on every call would produce approximately `1 + 2 + ... + n` work. `callProcessor` avoids this with `m_partialTokenBytesBefore` and `m_reparseDeferralEnabled`, enabled by default.

After a scan consumes nothing, the driver remembers the pending size. On non-final input it normally waits until pending bytes have at least doubled before scanning again. It also retries when the application's last buffer request would exceed available buffer space, allowing consumption before unnecessary reallocation. A successful consumption clears the remembered size. Final input always forces processing.

The consequences are observable: a non-final successful parse may defer callbacks even though the latest bytes complete a token. `XML_SetReparseDeferralEnabled` can change this policy, and child parsers inherit the setting. A replacement can instead implement truly incremental token scanning, but should explicitly decide how it matches this API option's callback timing.

## Hash collision resistance

DTD symbol tables and expanded-attribute duplicate checks use SipHash-2-4 from [`siphash.h`](../expat/lib/siphash.h). `startParsing` initializes a root salt before hash-dependent context setup. Descendants use the root salt so DTD copies and lookups remain consistent.

`generate_hash_secret_salt` prefers available platform entropy providers: arc4random facilities, Windows `rand_s`, getrandom, or `/dev/urandom` depending on build configuration. A time/process-derived fallback exists. `XML_SetHashSalt` allows an application-supplied salt before parsing. The actual key construction uses the root `unsigned long` salt, as described in [Data structures](data-structures-and-memory.md); do not assume the generic SipHash key size equals the entropy supplied here.

An alternative randomized map can satisfy the internal purpose, but it must protect both declaration/name lookup and namespace-expanded duplicate detection. Using a protected map for one and a predictable quadratic structure for the other leaves a separate worst-case path.

## Stack depth and integer arithmetic

Open elements are explicit `TAG` records. Internal content expansion uses entity frames and the processor reentry loop. Attribute and entity-value expansion each use explicit frame-processing loops. Content-model conversion is iterative. These choices keep input-controlled nesting out of the native call stack in those paths.

External entity handlers are application callbacks and can themselves create nested call chains, so these changes do not imply that all application-driven external loading is stack-bounded. Internal tag/entity/model cleanup should likewise remain iterative in a rewrite, including when language destructors would otherwise recurse through a linked structure.

Check sizes in their proper units before adding, multiplying, shifting, narrowing, or allocating. Frequent boundaries include input `int` lengths versus pointer differences; attribute count plus defaults; `XML_Char` counts times character size; namespace URI/local/prefix concatenation; table powers and capacity; model node/string totals; and pool growth. An out-of-memory error in this code often represents rejected size arithmetic, not only a failed allocator call.

## Diagnostic hooks and testable invariants

The implementation has `EXPAT_ACCOUNTING_DEBUG`, `EXPAT_ENTITY_DEBUG`, `EXPAT_MALLOC_DEBUG`, and `EXPAT_ENTROPY_DEBUG` diagnostics. Test builds expose accounting accessors, tracked allocation helpers, and `g_bytesScanned`. Diagnostic output is not ordinary callback output or a stable serialization format.

Useful invariants for a port are:

1. Resuming or rescanning pending input never repeats previously delivered semantic events or counts the same direct token twice.
2. Expanding through another child parser does not create a fresh uncharged budget.
3. An entity stays open until nested expansions finish, even after its own source text is exhausted.
4. A failed realloc leaves the original allocation and accounting valid.
5. Hash-table growth preserves interned object identity and uses consistent hashing across copied state.
6. Native stack use does not grow with internal entity or content-model depth.
7. Callback-free parsing still validates input and enforces resource limits; omitting handlers is not a bypass.
