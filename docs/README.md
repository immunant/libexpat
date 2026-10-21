# Expat codebase and reimplementation guide

Expat is an incremental XML parser that delivers events through application callbacks. Its core combines encoding-aware scanning, a state machine for the prolog and DTD, and a parser that manages element nesting, entity expansion, namespaces, and callback delivery. A compatible reimplementation must preserve the behavior at the boundaries between those layers, especially across input chunks, callbacks, and external entities.

This guide describes the C code in [`expat/lib/`](../expat/lib/) at commit `7cbe6c52c068dcb0d6b135f6e09cd6ff68c983fa`. The public header identifies this snapshot as **2.7.4**. It describes this checkout, rather than making claims about later releases. Other implementations and generated build trees elsewhere in the repository are outside its scope.

## Reading order

| Document | What it explains |
| --- | --- |
| [Architecture and source map](architecture.md) | Every core source file, layer boundaries, design choices, and build variants |
| [Parser execution](parser-execution.md) | Creation, buffering, processor transitions, callbacks, suspension, and a document walkthrough |
| [Tokenization and encodings](tokenization-and-encodings.md) | Scanner contracts, partial input, character tables, encoding detection, and conversion |
| [Data structures and memory](data-structures-and-memory.md) | Parser state, stacks, symbol tables, string pools, ownership, and cleanup |
| [DTD and entities](dtd-and-entities.md) | Grammar roles, declarations, normalization, internal expansion, and external parsers |
| [Attributes and namespaces](attributes-and-namespaces.md) | Attribute processing, defaults, namespace scope, expanded names, and duplicate checks |
| [API compatibility](api-compatibility.md) | Public API groups, callback contracts, positions, errors, reset behavior, and ABI |
| [Resource controls](resource-controls.md) | Expansion accounting, allocation tracking, hashing, and complexity invariants |
| [Reimplementation and verification](reimplementation-and-verification.md) | Suggested module boundaries, implementation sequence, test cases, and reference build commands |

Read the architecture and execution chapters first for a high-level map. For implementation work, follow the remaining chapters in order and use the verification chapter as an acceptance checklist. Source links and function names identify the implementation to consult for exact branches and error precedence; this guide complements the source and tests rather than reproducing every transition in prose.

## Compatibility target

There are three distinct targets to decide before implementation:

1. **XML behavior:** accepted input, normalized text, namespaces, entities, and declaration semantics.
2. **Expat API behavior:** callback ordering, chunk handling, errors, positions, suspension, configuration, and memory ownership.
3. **Binary replacement:** the C types, enum values, exported symbols, calling conventions, and build-dependent layouts.

A parser that reads ordinary XML successfully has only covered part of the first target. Expat is non-validating: it uses DTD information for entities, attribute defaults and normalization, and declaration callbacks, but does not validate the element tree against declared content models. It builds no document tree and performs no automatic external-resource retrieval.

The source implements several build variants. Unless otherwise stated, examples assume UTF-8 callback strings, general entities, DTD support, and namespace support compiled in. Namespace processing is enabled per parser only when requested. Parameter entity parsing defaults to `XML_PARAM_ENTITY_PARSING_NEVER`. [Architecture](architecture.md) describes the switches; [API compatibility](api-compatibility.md) distinguishes input encoding from callback encoding.

## Primary references in this checkout

- [`expat/lib/expat.h`](../expat/lib/expat.h) and [`expat/lib/expat_external.h`](../expat/lib/expat_external.h): public contract and types.
- [`expat/doc/reference.html`](../expat/doc/reference.html): existing API manual and configuration guidance.
- [`expat/tests/`](../expat/tests/): executable regression coverage, including allocation failures and accounting.
- [`expat/fuzz/`](../expat/fuzz/): fuzzing entry points for both input APIs and structured XML.
- [`expat/CMakeLists.txt`](../expat/CMakeLists.txt): build options, exported feature selection, and test targets.

When translating code or tables, preserve the applicable notices from the source files and [`COPYING`](../COPYING).
