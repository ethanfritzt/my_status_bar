# Module 4 — Turning battery state into status blocks

## Learn

- Modeling states with enums instead of magic strings.
- Optional values and safe arithmetic.
- Pure functions, threshold boundaries, and table-driven tests.
- Separating collection from presentation.

## Build it

1. Define a battery-state type that represents at least unavailable, discharging, and charging states. Convert parsed input into this type once, near the input boundary.
2. Calculate percentage only when the required values are valid and full capacity is non-zero. Decide how out-of-range values are handled.
3. Implement icon selection and color selection as pure functions. Preserve the current thresholds/icons initially, including the exact `> 50` and `> 25` boundaries.
4. Build the battery text and icon blocks using the block model from Module 2.
5. Add tests around 0%, 25%, just over 25%, 50%, just over 50%, and charging/unavailable states. Test exact boundary behavior, not only typical values.
6. Compare the formatted blocks with the Rust output for representative fixture inputs.

## Questions to answer in your notes

- Why is an enum safer than passing `"CHR"` and `"POW"` around?
- What should the percentage be when the battery is unavailable? Should there be a block at all?
- Is floating-point percentage appropriate, or would integer arithmetic be clearer for the display?
- Which existing behavior are you preserving, and which (if any) are you intentionally changing?

## Checkpoint

- Battery presentation functions have no filesystem or clock dependency.
- Boundary tests pass and cover each state.
- A fixture can be transformed into the expected pair of blocks.

## Hint (not a solution)

Keep parsing, state interpretation, and display formatting as separate steps. This makes a threshold bug testable without mocking sysfs or involving JSON.
