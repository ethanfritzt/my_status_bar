# Module 3 — Reading and parsing battery data

## Learn

- File and directory operations, slices, `defer`, and owned versus borrowed data.
- Error unions and the difference between a missing battery and a failed read.
- Parsing `key=value` records safely.
- Designing logic that can be tested without depending on the host's battery.

## Build it

1. Find the equivalent of the Rust collector in `src/utils/battery.rs`. List the exact sysfs files and keys it uses; do not start by porting every commented-out key.
2. Write a parser that accepts battery `uevent` text as input and extracts status, current energy, and full-design energy. Keep parsing separate from filesystem access.
3. Handle malformed lines (including lines without `=`), unknown keys, missing values, and invalid numbers without indexing past a slice or crashing.
4. Add tests using inline fixture text. Include normal data, a charging battery, missing keys, malformed lines, and invalid numeric values.
5. Add a filesystem layer that locates a battery under `/sys/class/power_supply`, checks its `type`, then reads its `uevent`. Keep paths injectable or keep the parser independently testable.
6. Decide explicitly what the program should do when the directory is missing, no battery exists, or a file cannot be read. Record whether you preserve or improve the Rust behavior.

## Questions to answer in your notes

- Which functions return owned data, and which return slices borrowing the input?
- At what point should an error become a non-fatal “battery unavailable” state?
- Is stopping at the first matching battery acceptable? What would multiple batteries mean?
- Can you test every parsing decision without access to `/sys`?

## Checkpoint

- Parsing tests pass using only in-memory strings.
- Bad input produces a controlled error or unavailable state, not a panic.
- The live collector works on the machine, and its failure behavior is understood.

## Hint (not a solution)

Prefer a pure parser with a small input/output contract. Parse each line with a first-separator operation, trim line endings intentionally, and distinguish “not present” from “present but malformed” where that distinction helps the caller.
