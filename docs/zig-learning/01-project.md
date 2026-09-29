# Module 1 — Project skeleton and workflow

## Learn

- How Zig compiles and runs a source file.
- What `pub fn main() !void` communicates about the entry point and errors.
- The difference between compile-time and runtime feedback.
- How to keep a small project easy to build without introducing dependencies.

## Build it

1. Create a separate `zig/` directory at the repository root. Do not replace or rename the Rust project.
2. Add a minimal `src/main.zig` that prints a short greeting, then exits successfully.
3. Compile and run it with Zig 0.16.0. Use the compiler's help output to find the relevant command options rather than assuming older-version instructions still apply.
4. Change the entry point to return an error when an output operation fails. Observe how error propagation changes the signature and call site.
5. Add a `build.zig` only after the direct compile/run workflow is understood. Make its default step build the executable.

## Questions to answer in your notes

- Where does the executable appear, and how can you choose its name/output path?
- What does `!void` mean here? What happens to an error that reaches `main`?
- Which standard-library API did you use for output, and where did you verify its Zig 0.16 signature?

## Checkpoint

- A clean build succeeds using Zig 0.16.0.
- Running the executable prints the greeting once and exits.
- The Rust sources and Cargo files are unchanged.

## Hint (not a solution)

Zig's I/O APIs have changed across releases. Treat the compiler version as part of the exercise: inspect `zig version`, then use that installation's library documentation/source. Don't cargo-cult `std.io.getStdOut()` examples written for a different release.
