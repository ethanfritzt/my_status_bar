# Learn Zig by Rewriting the Status Bar

A hands-on, incremental port of this project. The Rust implementation remains the behavioral reference; **write the Zig code yourself** and use each module's checkpoints before moving on. These modules provide tasks and guardrails, not a completed port.

## Version target

The environment available while preparing this course reports **Zig 0.16.0**. Use the same compiler for the course, or expect standard-library I/O and build APIs to differ between releases. Verify with `zig version`. When unsure about an API, consult documentation/source shipped with that exact compiler and make a tiny throwaway experiment rather than copying code for another Zig release.

## Suggested workflow

1. Read one module and write down what you expect the program to do.
2. Implement its tasks in small increments; keep the Rust version untouched.
3. Use `zig build-exe` / `zig test` and inspect output after each increment.
4. Check off the module only when its acceptance criteria pass.
5. Ask for a hint or review when stuck; avoid asking for the full implementation if you want the learning value.

## Course map

1. [Project skeleton and Zig workflow](01-project.md) — compile, run, and organize a tiny Zig program.
2. [Status blocks and JSON](02-status-blocks-json.md) — structs, defaults, optional fields, allocation, serialization.
3. [Battery sysfs parsing](03-battery-input.md) — filesystem input, slices, parsing, error handling, tests.
4. [Battery presentation](04-battery-display.md) — optionals, thresholds, enums, and mapping data to blocks.
5. [Clock block](05-clock.md) — time APIs, formatting, and isolating platform-dependent work.
6. [Protocol loop and final checks](06-runtime.md) — stdout protocol, composition, periodic updates, and integration.

## Scope and compatibility target

Match the current program first: emit the sway/i3 version header and an infinite JSON array stream; refresh about every three seconds; show battery icon/percentage and local time. Keep output behavior recognizable, but note any deliberate fixes in a short porting note. Do not add features or dependencies during the first pass.

The existing code is the reference in `src/`: `main.rs`, `status/block.rs`, and `utils/{battery,time}.rs`. A few quirks (such as its battery fallback and JSON framing) may be worth improving, but separate those changes from the language port so you can tell which differences come from Zig and which are behavior changes.

## Completion

The rewrite is complete when the Zig program builds with the pinned compiler, its output is accepted by sway/i3, battery and clock data work on the target machine, and the tests/checks in the modules pass. Keep the Rust version until you've run the Zig version for a while.
