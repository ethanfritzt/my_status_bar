# Module 6 — Protocol loop and integration

## Learn

- Composing independently tested components.
- Streaming protocol framing, output errors, and flushing.
- Separating one update from the infinite runtime loop.
- End-to-end verification against the original program.

## Build it

1. Write a function that creates one update by collecting battery blocks and a clock block. Test the composition/order separately from the loop.
2. Emit the sway/i3 header (`version: 1`) followed by the opening array marker exactly once.
3. Emit one JSON array of blocks per update, with the separators/newlines required by the i3bar streaming protocol. Check the protocol specification and the Rust implementation; don't assume a trailing comma is valid.
4. Flush stdout after each update so the bar does not wait on a buffer.
5. Put the delay in the runtime loop, not inside data collection. Keep the update function independently callable for tests.
6. Run the program in a terminal and inspect several updates. Then point a temporary sway/i3 config at the Zig executable and verify the actual bar renders. Keep the Rust executable available as a fallback.
7. Review all error paths: which failures should terminate the program, and which (for example unavailable battery data) should still allow the clock to render?

## Final checklist

- [ ] Builds with Zig 0.16.0 from a clean checkout/build directory.
- [ ] Header and streaming JSON framing are accepted by sway/i3.
- [ ] Refresh cadence is approximately three seconds.
- [ ] Battery and clock blocks render correctly.
- [ ] Parsing, thresholds, formatting, and JSON behavior have tests.
- [ ] No runtime status path relies on unchecked indexing or an unexplained panic.
- [ ] Any intentional differences from Rust are recorded in a short porting note.
- [ ] Rust sources remain intact until the Zig version has been used successfully.

## Reflection

Write a short note answering: Which Zig concepts felt most different from Rust? Where did explicit allocation/error handling help? Which part would you redesign now that the port is complete?
