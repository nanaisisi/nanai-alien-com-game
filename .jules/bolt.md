# Bolt Learnings

## 2023-10-27 - `rem_euclid` performance inside hot loops

**Learning:** In highly mathematical games like this 4X hex grid system, avoiding `%` for coordinate wrapping and opting for branching or `.rem_euclid()` only when truly out of bounds can drastically improve layout recalculation performance since it is on the hot path (every rendering frame and pathfinding step).
**Action:** Use fast path `if val < 0` / `val >= max` checks instead of blindly applying `.rem_euclid()` in spatial coordinates.

## 2025-02-14 - Dead Code in Mathematical Hot Paths

**Learning:** Found unused variable assignments resulting from expensive calculations in `map/hex.rs`. The code was calculating row and column representation including width via `to_col_row_with_width`, but only returning the final distance. The original implementation computes:
`let (_c1, _r1) = self.to_col_row_with_width(map_width);`
Notice `let (_c1, _r1)` is completely unused. Computing this takes time!
**Action:** Remove `self.to_col_row_with_width(map_width)` call inside `distance_with_width`. It's a pure dead-code performance bug.

## 2025-02-14 - Prevent UI Mutation Overhead

**Learning:** Bevy's change detection triggers whenever a mutable reference is dereferenced and assigned to (`*mut_ref = value;`), regardless of whether the value actually changed. This can cause unnecessary layout recalculations and system re-evaluations, which in an interactive game builds up rapidly every frame inside UI interaction query loops.
**Action:** Use conditional assignment (`if *mut_ref != new_value { *mut_ref = new_value; }`) or Bevy's `set_if_neq()` to prevent unnecessary change detection triggers when updating component state, particularly for `BackgroundColor` and `BorderColor` on UI buttons that are queried every frame.
