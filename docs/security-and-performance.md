# Resource limits and robustness

The protections here are part of the implementation's behavior, not optional
cleanup after implementing XML syntax. Sources are
[`xmlparse.c`](../expat/lib/xmlparse.c),
[`internal.h`](../expat/lib/internal.h), and
[`siphash.h`](../expat/lib/siphash.h). The defaults describe this checkout.

## Expansion accounting

With `XML_GE == 1`, the root parser owns `ACCOUNTING`:

- `D = countBytesDirect`: original root-document bytes counted as consumed.
- `I = countBytesIndirect`: bytes processed through entity expansion, including
  external child input and intermediate replacement work.
- Maximum amplification, activation threshold, and optional debug level.

Normally the measured amplification is `(D + I) / D`. If D is zero, the code
uses `(22 + I) / 22`, where 22 is the byte length of the shortest external-entity
declaration used as the fallback baseline. This supports isolated external
parser use without dividing by zero in expansion accounting.

Defaults are **100.0** maximum amplification and **8 MiB (8,388,608 bytes)**
activation threshold. Work is tolerated while `D + I` is below threshold;
at/above threshold the factor must be at most the maximum. This is not an
absolute output-size limit and does not count only application-visible text.
Nested intermediate text contributes even if the final callback output is small.

`accountingDiffTolerated` runs at scanner/semantic consumption sites. The account
category says DIRECT, ENTITY_EXPANSION, or NONE (already counted). DIRECT counts
as truly direct only on the root parser. Root lookup through `m_parentParser`
means external children cannot obtain separate unlimited budgets.

Incomplete/invalid/empty tokens are excluded before using the scanner's `next`
pointer, because that pointer may not have been set. In contexts that revisit
bytes, such as second-level normalization or the prolog-to-content transition,
the caller must avoid accidental double charging. Trailing CR/brackets and
predefined-entity output have explicit accounting treatment.

A failed expansion check returns `XML_ERROR_AMPLIFICATION_LIMIT_BREACH`.
Protection setters operate only on roots; factor setters reject NaN and values
below 1.0, while positive infinity is accepted by the implementation. Thresholds
can be set to zero. Counter additions have overflow guards. Exact boundary and
floating-point behavior should be retained or explicitly treated as a
compatibility difference.

## Allocation tracking is a different budget

`MALLOC_TRACKER` tracks current live **internal allocation bytes**, including
per-allocation metadata, for the root and descendants. Defaults are **100.0**
maximum allocation amplification and a **64 MiB (67,108,864 bytes)** activation
threshold.

Before a growing tracked allocation, compute its resulting live total `H`.
Below the threshold it is allowed by this budget. At/above threshold, the code
requires `H / D <= maximum`. D is the same processed direct-byte counter, not
the number of bytes submitted to public APIs. For D=0 and H>0, floating-point
division gives infinity, so a finite maximum rejects an activated allocation.

`expat_malloc` reserves a size header plus alignment padding, then the requested
payload. The returned pointer is offset past that header. `expat_free` recovers
size and raw pointer to decrement accounting before freeing. `expat_realloc`
checks a growth delta, updates sizes/totals only after success, handles null as
malloc, and handles zero as free. A failed realloc preserves the old allocation
and recorded size. Parser-object allocation needs special bootstrapping because
the tracker itself is inside that object.

An allocation budget violation appears as **allocation failure**: typically
`XML_ERROR_NO_MEMORY`, false, or null at the relevant API. It does not use the
entity amplification error. Cached pool blocks and free lists remain live and
counted; freed bytes decrease the budget. Reset does not simply zero the live
allocation tracker while cached memory remains.

These allocations deliberately bypass internal tracking and use the raw suite:

| Excluded allocation | Reason embodied in source |
| --- | --- |
| Input buffer | Application controls input size; `XML_GetBuffer` has separate capacity/overflow checks |
| Application-owned `XML_Content` result | Preserves freeing compatibility with the application's allocation suite |
| `XML_MemMalloc/Realloc/Free` calls | Application memory is outside the internal allocation budget |

Thus the tracker is not a cap on process memory, input bytes, callback-retained
objects, or all memory returned through Expat APIs. A replacement must preserve
allocation-family matching, even if it implements budget metadata differently.

## Hash collision hardening

Persistent name tables and expanded-attribute duplicate tables use SipHash-2-4.
`copy_salt_to_sipkey` derives the SipHash key from the root parser's secret salt;
this is not a separately generated 128-bit secret. A root initializes the salt
at parse start unless the application supplied a nonzero one. Descendants use
the same root secret so copied tables can be queried consistently.

Entropy selection uses configured system facilities such as `arc4random_buf`,
`arc4random`, `getrandom`/syscall, `/dev/urandom`, or Windows `rand_s`. Runtime
fallback uses time/process information when higher-quality attempts fail.
Builds without a configured high-quality facility require explicit
`XML_POOR_ENTROPY` to bypass the compile-time diagnostic. A replacement should
use a suitable local random facility rather than a fixed production key.

Tables use power-of-two capacities, low load factors, and odd probing steps.
Hashing protects expected performance against crafted name collisions; it does
not replace duplicate string checks or allocation limits. Test runs can set a
fixed salt for reproducibility, without treating hash-table iteration order as
XML document order.

## Reparse deferral

A scanner often needs a complete comment, start tag, PI, or literal before it
can emit a token. If each new byte makes it rescan an ever-longer unfinished
token, total work is quadratic even though the token itself is legal.

`callProcessor` remembers the unconsumed span size after a no-progress attempt.
For nonfinal input with deferral enabled (the default), it waits until the span
roughly doubles before retrying. It may retry earlier when the latest requested
fill size exceeds reusable free capacity, avoiding needless buffer growth.
Final input always parses immediately. Successful progress clears the history.

The tradeoff is observable callback latency: an API call may succeed without
reporting newly complete tokens yet. `XML_SetReparseDeferralEnabled` is an
explicit option copied to external children and mutable on the fly. Disabling
it is not a benign performance-only change for hostile large tokens.

CDATA and normal text can stream in smaller runs, whereas a very large comment
or attribute may require retaining a large unfinished token. Deferral improves
scan work; it does not impose a maximum token size or eliminate buffer growth.

## Avoiding native stack exhaustion

Internal entity expansion uses explicit lists/continuations in all three
contexts: content/prolog, attributes, and entity declaration values. A parent
must remain open until its scheduled descendants close, preventing cycles from
being hidden by delayed work. Entity statistics record depth but do not impose
a standalone fixed maximum depth in this code.

Open elements also use a heap-linked stack. Content-model construction flattens
scaffolding iteratively instead of recursively walking untrusted nesting. A
replacement that uses recursive descent for every nested element, group, or
replacement can reintroduce exhaustion that the current implementation avoids.
Application-controlled chains of external callback invocations still require
appropriate application behavior; not all application stack use is governed by
the internal entity loop.

## Overflow and failure discipline

Preserve checks around:

- Adding requested input bytes, buffered remainder, and context; growing buffer
  capacities; advancing absolute byte indices.
- Multiplying element/attribute/node counts by object or character widths.
- Adding URI, local name, prefix, terminator, and spare capacity lengths.
- Doubling string blocks, tag buffers, declaration arrays, group workspace, and
  hash capacities; validating shifts before shifting.
- Allocator header/padding sizes, allocation deltas, and byte counters.
- Rebased pointers after realloc and borrowed pointers into input or URI buffers.

Use checked arithmetic before conversion to narrower signed fields. Leave an
object valid for free/reset on all failure paths, including failed child creation
with a shared DTD, failed unknown decoder installation, and allocation failure
halfway through attribute/namespace assembly. Assertions supplement runtime
checks; they cannot be the only protection against malformed input.

## Complexity and observability

For ordinary input, token scanning/conversion is proportional to processed
bytes and interned-name lookups are expected constant-time per lookup, plus
name hashing/comparison cost. Entity expansion can be much larger than source
input but is budgeted. Internal storage includes declarations/distinct names,
maximum active nesting, reusable caches, and the largest buffered token; there
is no unconditional constant-memory guarantee.

Specific paths can rescan attributes after workspace growth, linearly inspect
an element's declarations, or repair several open-tag pointers when a URI buffer
grows. Avoid promising a strict global linear bound based only on the reparse
heuristic. Measure those paths separately if optimizing a replacement.

Diagnostic environment variables include `EXPAT_ACCOUNTING_DEBUG`,
`EXPAT_MALLOC_DEBUG`, `EXPAT_ENTITY_DEBUG`, and `EXPAT_ENTROPY_DEBUG`. Testing
builds expose scan/accounting helpers. Exact diagnostic output is separate
from the public XML event API, but the underlying counters are useful for
performance regression tests.
