# Expat code map and reimplementation guide

This guide describes the C implementation in [`expat/lib/`](../expat/lib) at
repository commit `7cbe6c52c068dcb0d6b135f6e09cd6ff68c983fa`. The public header
reports **2.7.4**; source banners say **2.7.4+**. Statements about defaults and
implementation details refer to this checkout, not to a moving upstream release.

Expat is an incremental, callback-driven XML parser. It recognizes markup,
checks well-formedness, processes declarations and entity replacement text,
normalizes values, optionally expands namespace names, and delivers events.
It does not construct a document tree or validate document content against a
DTD. Applications supply input and resolve external resources.

## Reading paths

| Document | What it explains |
| --- | --- |
| [Architecture and source map](architecture.md) | Every library source file, module boundaries, design choices, and a function-level navigation map |
| [State, storage, and ownership](data-structures.md) | Parser object, tags, entities, declarations, namespace bindings, string pools, hash tables, and allocation lifetimes |
| [Streaming parser walkthrough](parsing.md) | Input APIs, processor transitions, token dispatch, finalization, suspension, positions, and worked event traces |
| [Tokenization and encodings](tokenization.md) | Scanner contracts, generated scanner variants, byte classes, detection, conversion, XML declarations, and name rules |
| [DTD and entity processing](dtd-and-entities.md) | Role grammar, declarations, defaults, parameter entities, external parsers, iterative expansion, and content models |
| [Attributes and namespaces](attributes-and-namespaces.md) | Attribute pipeline, normalization, duplicate detection, namespace scope, reserved bindings, and expanded names |
| [Public API and build compatibility](api-and-build.md) | API inventory, callback contracts, ABI, feature switches, reset behavior, and feature reporting |
| [Resource limits and robustness](security-and-performance.md) | Entity amplification, allocation tracking, hash hardening, reparse deferral, overflow checks, and complexity |
| [Reimplementation plan and verification](reimplementation.md) | Implementation stages, invariants, compatibility matrix, differential testing, and existing regression/fuzz infrastructure |

For an initial orientation, read architecture, data structures, and parsing.
For implementation work, continue through the remaining chapters; each covers
behavior that can change observable results even on small documents.

## Scope and use

The guide distinguishes **observed implementation behavior**, **public API
contracts**, and **suggested replacement design**. A replacement may use different
containers, ownership mechanisms, or state representations while preserving the
required external behavior. Internal structure layout usually is not an ABI
contract; the `XML_GetUserData` macro is a notable exception.

These documents provide the design map and implementation checklist. The exact
enums, exported signatures, character tables, grammar transitions, error strings,
and platform ABI remain defined by the linked source. Use them, the checked-in
regressions, and [`expat/doc/reference.html`](../expat/doc/reference.html) when
resolving a compatibility detail. XML grammar coverage cannot be established by
the worked examples alone.

The library sources carry MIT notices; repository license texts are in
[`COPYING`](../COPYING) and [`expat/COPYING`](../expat/COPYING). Retain the
applicable notices when incorporating existing source or tables into a port.

All source links are relative to this directory. Symbol names are the primary
navigation keys, so references remain useful when line numbers change. Build
and test commands are reproducible recipes; checks actually performed for this
guide are listed below.

## Verification of this guide

An out-of-tree CMake build of this checkout, with its default UTF-8/GE/DTD/NS
features and tests enabled, passed both CTest targets (`runtests` and
`runtests_cxx`). Source-built event probes checked the illustrated normalization,
namespace ordering/triplets, nested entity output, and finalization behavior;
normalization was also checked with several chunk sizes. The entity probe
identified the header/implementation byte-count discrepancy documented in
[the parsing walkthrough](parsing.md).

All local documentation links and named regression references were checked, and
every public API function name appears in the guide. These checks validate the
documentation against this source build; they are not a compatibility result
for a new implementation or for every feature configuration.
