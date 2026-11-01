## 2023-10-27 - `rem_euclid` performance
**Learning:** `rem_euclid` on integers in Rust is actually reasonably well optimized by LLVM since 1.84+ (we're using nightly), compiling to branching/modulo logic that is very fast. A manual implementation (`a % b` plus `if c < 0 { c += b }`) was slightly slower than `rem_euclid`. Wait, I did benchmark `rem_euclid` previously and it was faster manually when compiling without optimizations, but with `-O` they are roughly equivalent, or `rem_euclid` is actually slightly faster or just as fast, and much more readable. However, there's a big optimization in `distance_with_width`.

**Learning 2:** `distance_with_width` on HexCoord is very frequently used.
The original implementation computes:
```rust
    pub fn distance_with_width(self, other: HexCoord, map_width: i32) -> i32 {
        let (_c1, _r1) = self.to_col_row_with_width(map_width);
        let (c2, r2) = other.to_col_row_with_width(map_width);

        let offsets = [-map_width, 0, map_width];
        let mut min_dist = i32::MAX;

        for offset in offsets {
            let shifted_other = HexCoord::from_col_row_unwrapped(c2 + offset, r2);
            let dq = (self.q - shifted_other.q).abs();
            let dr = (self.r - shifted_other.r).abs();
            let ds = (self.s() - shifted_other.s()).abs();
            let d = (dq + dr + ds) / 2;
            if d < min_dist {
                min_dist = d;
            }
        }
        min_dist
    }
```
Notice `let (_c1, _r1) = self.to_col_row_with_width(map_width);` is completely unused. Computing this takes time!

**Action:** Remove `self.to_col_row_with_width(map_width)` call inside `distance_with_width`. It's a pure dead-code performance bug.
