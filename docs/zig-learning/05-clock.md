# Module 5 — The clock block

## Learn

- Looking up standard-library capabilities before choosing an implementation.
- Dealing with platform-specific APIs and formatting boundaries.
- Keeping time acquisition separate from block formatting.

## Build it

1. Find the current Rust behavior: local time formatted as 24-hour `HH:MM`.
2. Investigate what Zig 0.16.0 provides for getting local time and formatting it. Write down whether the solution is standard-library-only, uses libc, or uses another system interface. Do not add a dependency without first discussing it.
3. Implement the smallest function that turns a supplied time value into `HH:MM`, then test formatting independently from reading the system clock.
4. Implement live local-time acquisition and construct the clock status block.
5. Verify behavior at midnight/noon and confirm the displayed value follows local time rather than UTC.

## Questions to answer in your notes

- Which part is pure and testable, and which part depends on the operating system clock?
- What timezone does your chosen API return?
- What portability or linking cost did your choice introduce?
- If Zig's standard library doesn't provide a convenient local-time API, what are the trade-offs of libc versus invoking an external command? Prefer not to spawn a command every refresh.

## Checkpoint

- The formatter can be tested with known values without sleeping or changing the system clock.
- The live block displays local time in the same format as the Rust reference.
- Any platform or libc assumption is documented.

## Hint (not a solution)

Don't make formatting tests depend on “whatever time it is now.” Pass a known time representation into the formatter and test the acquisition layer separately.
