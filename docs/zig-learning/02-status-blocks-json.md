# Module 2 — Status blocks and JSON

## Learn

- Structs, fields, string slices, and `?T` optional values.
- Defaults and the trade-offs between explicit constructors and default initialization.
- JSON serialization and allocator ownership.
- Why protocol output should be tested as data, not just eyeballed as text.

## Build it

1. Define a Zig status-block type representing the fields in `src/status/block.rs`. Decide which fields are required and which may be absent. Preserve the Rust output shape for this first pass.
2. Create a constructor/default strategy that makes a basic block easy to create while allowing selected fields to be overridden. Avoid silently sharing mutable buffers.
3. Create a small executable that builds one block, serializes it as JSON using the Zig 0.16 standard library, and writes it to stdout.
4. Inspect the JSON. Check string escaping, booleans, numbers, and how an absent `short_text` is represented. Adjust the type/serialization policy to match the Rust reference.
5. Add a test for serialization. Parse the emitted JSON back into a value or compare its meaningful fields; avoid a brittle full-string test if key ordering is unspecified.

## Questions to answer in your notes

- Who owns the serialized JSON bytes, and how long are they valid?
- Which allocator did you choose at this stage, and why is that appropriate for a short-lived serialization buffer?
- Does an absent optional field serialize as `null` or get omitted? Which behavior matches the existing program?
- What happens if serialization or the stdout write fails?

## Checkpoint

- One status block serializes to valid JSON with the expected field types.
- The optional field follows the chosen protocol-compatible policy.
- Tests cover at least a normal block and an absent optional value.

## Hint (not a solution)

Read the Zig 0.16 `std.json` API rather than relying on examples for another release. Keep the in-memory model separate from protocol decisions if the serializer's defaults don't match the existing output.
