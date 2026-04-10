The code under `rexpat/` was translated using C2Rust
from the C code under `expat/`. Your goal is to refactor
the Rust code to make it as idiomatic as safe as possible.

Core goals
* The final code should be as idiomatic as possible and use
  Rust idioms and standard library types and cratesi from https://blessed.rs
  as much as possible
* There should be no unsafe Rust code or raw pointers left except
  where required by the FFI or absolutely necessary (document every such case)
* The final library should be ABI compatible with the C one, and
  all publicly visible functions and types should be left untouched.
* You will run for hours: plan first, then implement milestone by milestone.
  Do not skip the planning phase.
* The refactoring should focus on the rexpat library crate; tests can
  be refactored as well but this is not strictly necessary. In either case,
  the tests should always use the C interface to the library (do not refactor
  the testing code into safe Rust imports and calls into rexpat).

Constraints
* Do not stop after a milestone to ask me questions or wait for confirmation.
* Proceed through every milestone in `docs/plans.md` until the whole project is complete and fully validated.

Process requirements (follow strictly)

1. PLANNING FIRST (write this file before coding anything):

   * Create `plans.md` with a milestone plan that will take hours.
   * For each milestone include: scope, key files/modules, acceptance criteria, and commands to verify.
   * Include a “risk register” with top technical risks and mitigation plans (safety, performance, correctness, idiomaticity).

3. IMPLEMENT NEXT:

   * Implement one milestone at a time.
   * After each milestone: run verification commands, fix issues, commit with a clear message.
   * Keep diffs reviewable and avoid giant unstructured changes.

3. If you hit complexity choices:

   * Prefer correctness, then safety, then idiomaticity, then performance.
   * Document tradeoffs and decisions in `docs/plans.md` as you go.

Start now.
First, create `docs/plans.md` with the complete plan, risk register, and milestones.
Use Codex subagents where they would be useful.
Do NOT start coding until `plans.md` exists and is coherent; stop and let me
review the file once it is ready.
