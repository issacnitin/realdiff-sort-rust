# RealDiff Rust sort-stability demo

RealDiff runs the same tests on both sides of this pull request and reports the runtime values that changed.

## How it works

1. Check out the base and pull-request revisions.
2. Build both through RealDiff's stable Rust source rewriter.
3. Run the same Cargo tests on both, recording observed function arguments and return values.
4. Diff those execution traces instead of inferring behavior from the source diff.

This is not mutation testing, static analysis, or coverage. No production code or test is mutated in the repository, RealDiff does not generate tests, and it observes only code this test suite executes.

## Worked example

The pull request opts into Rust's potentially faster unstable sort. In this block, `-` is the base, `+` is the proposal, and the single important change is `sort_by` becoming `sort_unstable_by`:

```diff
-$rules.sort_by(|left, right| left.priority.cmp(&right.priority));
+$rules.sort_unstable_by(|left, right| left.priority.cmp(&right.priority));
```

Both calls sort by priority, so the edit looks like a local performance refactor. The stable base preserves declaration order and selects `A_SEASONAL`. The unstable sort rearranges equal-priority entries and selects `Z_CLEARANCE` in this deterministic fixture.

The following block labels the exact values RealDiff observed before and after the edit:

```text
BASE  select_discount(100) -> A_SEASONAL
PR    select_discount(100) -> Z_CLEARANCE
BASE  checkout_total(100) -> (85, A_SEASONAL)
PR    checkout_total(100) -> (60, Z_CLEARANCE)
```

Neither pricing function is in the diff; only `src/config.rs` changed. All three tests execute the path. The two broad total assertions still pass because 60 is discounted and does not exceed 100. Only `seasonal_discount_wins_current_ties`, which checks the exact selected code, reacts.

## Why the finding is focused

RealDiff runs the base more than once and subtracts observations that disagree with themselves, removing timestamps, GUIDs, hash-order variation, and similar self-noise.

The changed rule affects its callers, but RealDiff collapses that propagation and reports the first changed behavior in unedited `src/pricing.rs`.

## Run it

The command below runs the demo's three tests:

```bash
cargo test
```
