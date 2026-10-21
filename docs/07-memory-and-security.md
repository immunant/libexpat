# 7. Memory management and security hardening

Expat parses untrusted input in browsers, language runtimes (Python, Ruby,
PHP, Perl), and embedded systems. Much of its recent complexity comes from
defending against denial of service and memory-safety bugs. This document
gathers those mechanisms in one place. A compatible reimplementation should
provide the same protections with the same defaults.

## 7.1 Pluggable allocation and the allocation tracker

`XML_ParserCreate_MM(enc, memsuite, sep)` lets the application supply
`malloc_fcn`, `realloc_fcn`, and `free_fcn`. They are stored in `m_mem`
(declared `const`, written once through a cast in `parserCreate`). Child
parsers inherit them.

Internally every allocation goes through these macros:

```c
#if XML_GE == 1
#  define MALLOC(parser, s)      expat_malloc((parser), (s), __LINE__)
#  define REALLOC(parser, p, s)  expat_realloc((parser), (p), (s), __LINE__)
#  define FREE(parser, p)        expat_free((parser), (p), __LINE__)
#else
#  define MALLOC(parser, s)      (parser->m_mem.malloc_fcn((s)))
...
```

With `XML_GE == 1`, **`expat_malloc` puts a hidden header in front of every
block** (`xmlparse.c:851`):

```
 mallocedPtr                                   returned pointer
 │                                             │
 ▼                                             ▼
 [ size_t size ][ padding to alignof(long long) ][ user bytes ... ]
```

`EXPAT_MALLOC_PADDING = sizeof(long long) - sizeof(size_t)` (`internal.h`).
The header lets `expat_free`/`expat_realloc` know the block size, so the
**root parser** can keep `m_alloc_tracker.bytesAllocated` exact.
`parserCreate` lays out the parser struct the same way, so even the parser
itself is counted.

Before every growth, `expat_heap_increase_tolerable(root, increase)` checks:

```
newTotal = bytesAllocated + increase              (with overflow check)
if newTotal >= activationThresholdBytes (default 64 MiB):
    if newTotal / countBytesDirect > maximumAmplificationFactor (default 100.0):
        refuse  →  the allocation returns NULL  →  XML_ERROR_NO_MEMORY
```

Here `countBytesDirect` is the number of input bytes the application has fed
to the root parser (see 7.2). Heap use is therefore limited **relative to
input size**: a small document cannot make Expat allocate gigabytes. The
limits can be changed with `XML_SetAllocTrackerMaximumAmplification` and
`XML_SetAllocTrackerActivationThreshold` (on the root parser only).

**Deliberately not tracked** (they use `m_mem.*_fcn` directly):

* the **input buffer** `m_buffer`, because the application decides how much
  input to send;
* **content models** given to `ElementDeclHandler`, so applications may free
  them with plain `free`;
* `XML_MemMalloc`/`XML_MemRealloc`/`XML_MemFree`, which are for the
  application's own use.

Mixing these up would corrupt the heap, because a tracked block's pointer is
offset from the real allocation. The comments at each such site say so.

## 7.2 Billion-laughs protection (amplification accounting)

**The attack:** nested entity definitions such as
`<!ENTITY a "&b;&b;&b;…">` grow exponentially when expanded, so a few
hundred bytes of input can produce gigabytes of output.

**The defense** (`XML_GE == 1`, `xmlparse.c:8546`): every token the parser
processes is passed to

```c
XML_Bool accountingDiffTolerated(parser, tok, before, after, line, account);
```

where `account` is one of:

| `enum XML_Account` | Meaning |
|--------------------|---------|
| `XML_ACCOUNT_DIRECT` | bytes from the application's input (only counted as direct when `parser` is the root) |
| `XML_ACCOUNT_ENTITY_EXPANSION` | bytes produced by expanding an entity (internal entity text, external entity input in child parsers) |
| `XML_ACCOUNT_NONE` | already counted, skip |

The **root** parser's `m_accounting` keeps `countBytesDirect` and
`countBytesIndirect`. After each addition:

```
amplification = (direct + indirect) / direct
tolerated = (direct + indirect) < activationThresholdBytes   (default 8 MiB)
            || amplification <= maximumAmplificationFactor   (default 100.0)
```

If it is not tolerated, the parser fails with
`XML_ERROR_AMPLIFICATION_LIMIT_BREACH`. Bytes from child parsers count as
indirect even when they are the child's own direct input, because they were
pulled in by an entity reference. Expanding a predefined entity like `&lt;`
is counted without a check, since replacing 4–6 bytes with 1 cannot amplify.

`accountingDiffTolerated` calls appear at the top of every token loop:
`doContent`, `doProlog`, `doCdataSection`, `doIgnoreSection`,
`appendAttributeValue`, `storeEntityValue`, `epilogProcessor`,
`processXmlDecl`, and the BOM handling in the init processors. Some calls pass
`XML_ACCOUNT_NONE` on purpose to avoid counting the same bytes twice. For
example, the attributes of an empty-element tag are inside a token that was
already counted. Getting this exactly right matters, because the
`testingAccountingGetCountBytes*` hooks are checked by the test suite.

**The default thresholds are part of the behavior** that applications rely
on (`internal.h`):

| Constant | Value |
|----------|-------|
| `EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT` | `100.0f` |
| `EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT` | `8388608` (8 MiB) |
| `EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT` | `100.0f` |
| `EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT` | `67108864` (64 MiB) |

They are also exposed through `XML_GetFeatureList()` as `XML_BLAP_MAX_AMP`,
`XML_BLAP_ACT_THRES`, `XML_AT_MAX_AMP`, and `XML_AT_ACT_THRES`. The setters
reject NaN, factors below 1.0, and calls on child parsers.

**Diagnostics:** the environment variables `EXPAT_ACCOUNTING_DEBUG`,
`EXPAT_MALLOC_DEBUG`, `EXPAT_ENTITY_DEBUG` (levels 1–3), and
`EXPAT_ENTROPY_DEBUG` make Expat print a trace to `stderr`
(`getDebugLevel`, `xmlparse.c:9208`). Entity tracking
(`entityTrackingOnOpen/Close`) records how many entities were opened and the
nesting depth, and exists only for these diagnostics.

## 7.3 Hash-flooding protection

**The attack:** with a known hash function, an attacker can pick element,
attribute, or entity names that all collide. Each `lookup` then becomes
O(n), and parsing becomes O(n²).

**The defense:**

* All hash tables use **SipHash-2-4** (`siphash.h`), a keyed PRF, with key
  `(0, salt)`. Expanded-attribute duplicate detection (`m_nsAtts`) uses it
  too.
* The salt is a random `unsigned long` from `generate_hash_secret_salt()`,
  created on the first `XML_Parse*` call (`startParsing`) unless the
  application set one with `XML_SetHashSalt`. Entropy sources, in order of
  preference: `arc4random_buf` / `arc4random`, then `rand_s` (Windows) or
  `getrandom(GRND_NONBLOCK)` / `SYS_getrandom`, then `/dev/urandom`, then a
  weak fallback (`gettimeofday` XOR pid, times a Mersenne prime). The build
  **refuses to compile** without a strong source unless `XML_POOR_ENTROPY` is
  defined.
* Child parsers use the **root's** salt (`get_hash_secret_salt` goes through
  `getRootParserOf`), so tables copied by `dtdCopy` stay consistent.

## 7.4 Reparse deferral

**The problem:** because the tokenizer is stateless (see
[02 §2.1](02-tokenizer.md#21-the-scanner-contract)), a token split across
many small `XML_Parse` calls is rescanned from the start each time. A large
token (a long attribute value or comment) sent 1 byte at a time costs
O(n²).

**The fix** (`callProcessor`, Expat 2.6.0): if the previous attempt consumed
nothing (`m_partialTokenBytesBefore = bytes we had`), do not rescan until the
available data has at least **doubled**:

```c
if (m_reparseDeferralEnabled && !finalBuffer) {
  enough = (have_now >= 2 * had_before)
        || (m_lastBufferRequestSize > available_buffer_space);  // about to realloc anyway
  if (!enough) { *endPtr = start; return XML_ERROR_NONE; }       // pretend we made no progress
}
```

Rescans are then O(log n) per token. The second condition stops deferral
from causing unnecessary buffer growth. `XML_SetReparseDeferralEnabled(p,
XML_FALSE)` turns it off for applications that need every handler call as
soon as possible, such as interactive streaming. `isFinal` always forces a
full parse.

## 7.5 Removing recursion

Input-controlled recursion was rewritten as iteration in two places:

| Old (recursive) | New (iterative) | Ref |
|-----------------|-----------------|-----|
| `build_node` for content models | `build_model` breadth-first with an in-array job queue | [05 §5.3](05-dtd-and-entities.md#element-declarations-and-content-models) |
| `processInternalEntity` → `doContent`/`doProlog` → … | explicit open-entity stacks + `m_reenter` loop in `callProcessor`, plus loops in `storeAttributeValue` and `callStoreEntityValue` | [05 §5.4](05-dtd-and-entities.md#54-expanding-internal-entities-without-recursion) |

What remains bounded but recursive: external entities. A handler calls
`XML_Parse` on a child, which may call the handler again. The depth is
limited by entity nesting in the document, and recursion through the same
entity is caught by the `open` flags passed via `getContext`. The
application controls this path.

`callProcessor` also has a rule for a suspend or abort that happens while a
re-enter is pending: the parsing status wins, and the re-enter flag is
cleared.

## 7.6 Arithmetic and other defensive idioms

A reimplementation in a language with checked arithmetic gets most of these
for free. In C, Expat does the following:

* **Every growth computation is checked before use**:
  `if (n > INT_MAX - m) return XML_ERROR_NO_MEMORY;`,
  `if (count > SIZE_MAX / sizeof(T))`, and doubling done as
  `(int)(2U * (unsigned)x)` followed by a sign check. Integer overflow is
  reported as out of memory.
* **Shift amounts are checked** (`m_nsAttsPower`, hash `power`) before
  `1u << power`.
* `XML_Parse` rejects `len < 0` and `s == NULL && len != 0`
  (`XML_ERROR_INVALID_ARGUMENT`), and stops `m_parseEndByteIndex` from
  overflowing.
* `XML_ParseBuffer` without a prior successful `XML_GetBuffer` gives
  `XML_ERROR_NO_BUFFER`.
* Namespace URIs containing the separator character are rejected when the
  separator is not a URI character. This prevents ambiguous expanded names
  (`is_rfc3986_uri_char`).
* When converting to UTF-8/UTF-16, multibyte characters are never split
  across output buffers (`_INTERNAL_trim_to_complete_utf8_characters`).
* Pointer arithmetic that might involve NULL buffer pointers uses
  `EXPAT_SAFE_PTR_DIFF`.
* `LCOV_EXCL` blocks keep "impossible" checks rather than deleting them.

## 7.7 Threading

A parser object is **not** thread-safe, and handlers run on the caller's
thread. Separate parsers share no mutable global state. The only globals are
read-only tables and `g_reparseDeferralEnabledDefault`, which is `const`
outside test builds. So separate parsers may run in parallel. The comment on
`XML_ExternalEntityParserCreate` says a general-entity child parser "is
completely independent and may safely be used in a separate thread". In this
version that is **no longer strictly true** when `XML_GE == 1`. The child has
its own DTD copy, but `expat_malloc`, `accountingDiffTolerated`, and the
entity statistics all update counters in the *root* parser without any
synchronization. Running a child concurrently with its root, or with
siblings, races on those counters. A reimplementation should either use
atomic counters or document that child parsers are tied to their root's
thread.
