# **What bead sort is (core idea — no fluff)**

**Bead sort** (a.k.a. *gravity sort*, *abacus sort*) is a **non-comparison** sorting method that models numbers as beads on horizontal rods and lets the beads “fall” under gravity. After the fall, reading the vertical columns produces the numbers in sorted order. It works only for **non-negative integers** (or values mapped to non-negative integers).

---

# **Intuition / physical model**

* Represent each element `A[i]` as a row of `A[i]` beads placed on an abacus with `n` vertical rods (one rod per element).
* Let beads fall down: each *row* of beads will collect at the bottom, filling some rightmost rods.
* After falling, the height of beads on each vertical rod is the sorted sequence (ascending if you read left→right; descending if you read right→left depending on orientation).
* **Analogy:** Each horizontal level `r` corresponds to "which elements are ≥ r". After gravity, the bottom `C[r]` rods are occupied where `C[r]` = number of elements ≥ `r`.

---

# **Two canonical algorithmic views**

1. **Naïve (matrix) simulation**

   * Build an `m x n` boolean matrix where `m = max(A)`. Row `r` has `true` at column `i` if `A[i] >= r+1`.
   * For each row compute how many `true`s it contains, then place that many beads at the bottom of that row (rightmost columns).
   * Read column heights → sorted array.
   * **Time / space:** `O(n * m)` time and space (matrix size), where `m = max(A)`.

2. **Histogram / cumulative (optimized, practical)**

   * Build histogram `H[v] = count of elements equal to v`.
   * Compute cumulative `C[r] = number of elements ≥ r` for `r = 1..m`.
   * For each row `r`, you know exactly `C[r]` beads will fill the bottom `C[r]` columns; increment the height of those columns.
   * Implementation complexity ≈ `O(n + m + S)` where `S = sum(A)` (total beads). Space `O(n + m)`.
   * This is the usual *digital* implementation of bead sort and is the best tradeoff for code.

---

# **Correctness sketch**

* Row `r` initially marks all elements with `A[i] ≥ r`. After gravity that row fills the `C[r]` rightmost columns.
* The resulting height of column `j` equals the number of rows `r` that put a bead in column `j`, i.e. `height[j] = |{ r : column j is among the bottom C[r] }|`.
* This resolves to `height[j] = |{ r : C[r] ≥ n-j }|`, giving monotonic increasing heights left→right — therefore sorted order.

---

# **Limitations & **must-know** constraints**

* **Domain:** Only integer (non-negative) values. To handle negatives, shift all values by a constant offset. To handle floats, quantize/scale to integers.
* **Memory:** Naïve matrix uses `O(n*m)` memory; for big `m` this is impractical.
* **Time:** Optimized is `O(n + m + S)`. If `S` (sum of elements) is huge, algorithm is slow.
* **Not a comparison sort:** It can beat `O(n log n)` in settings where `m` and `S` are small compared to `n`.
* **Stability:** Not stable (beads lose identity).
* **Edge cases:** empty list, zeros-only, very large values → watch for overflow and memory exhaustion.

---

# **When to use bead sort (practical advice)**

* Use when:

  * All values are small non-negative integers (or can be mapped to such).
  * `S` (sum of values) is reasonably small.
  * You want a conceptually parallel / hardware-friendly algorithm.
* Prefer counting sort or radix sort when you need simpler and often faster digital sorts:

  * Counting sort: `O(n + k)` where `k` = value range.
  * Radix sort: good for integers of bounded word size.
* Bead sort shines conceptually for **physical parallelism** (actual beads falling simultaneously) or as a pedagogical/novel algorithm.

---

# **Complexity summary (table)**

| Algorithm                  |     Time (typical) |           Space | Notes                      |
| -------------------------- | -----------------: | --------------: | -------------------------- |
| Bead (naïve matrix)        |         **O(n·m)** |      **O(n·m)** | `m = max(A)`               |
| Bead (histogram/optimized) |   **O(n + m + S)** |    **O(n + m)** | `S = sum(A)` (total beads) |
| Counting sort              |       **O(n + k)** |        **O(k)** | `k = range`                |
| Radix sort                 |       **O(n · d)** | **O(n + base)** | `d = digits`               |
| Quick sort (comparison)    | **O(n log n)** avg |    **O(log n)** | Comparison lower bound     |

---

# **Practical optimizations**

* Use **histogram+cumulative** approach (avoids `n·m` memory).
* Use **bitsets** / word-packing to compress rows if implementing matrix; helps memory and bit-ops can speed up.
* Parallelize per-row updates; each `C[r]` is independent — excellent for SIMD/GPU/FPGA.
* Check and guard for integer overflow: `S` may be > 32-bit; use 64-bit counters.
* For very large `m` or `S`, prefer counting/radix sort.

---

# **Implementations**

Below are clear, commented implementations showing **naïve** (for conceptual clarity) and **optimized** (practical) bead sort. Each implementation includes edge-case handling and test examples.

---

## C++ — Naïve matrix simulation (educational)

```cpp
// naive_bead_sort.cpp
// Conceptual bead sort: O(n * m) time and space. Only for small arrays or teaching.

#include <bits/stdc++.h>
using namespace std;

// Sorts ascending. Works for non-negative ints.
vector<int> bead_sort_naive(const vector<int>& a) {
    int n = (int)a.size();
    if (n == 0) return {};

    int m = *max_element(a.begin(), a.end());
    if (m == 0) return vector<int>(n, 0);

    // beads[row][col] where row 0..m-1 (bottom->top), col 0..n-1 (left->right)
    vector<vector<char>> beads(m, vector<char>(n, 0));

    // Place beads: for element a[col], set beads[0..a[col]-1][col] = 1
    for (int col = 0; col < n; ++col) {
        for (int r = 0; r < a[col]; ++r) beads[r][col] = 1;
    }

    // Simulate gravity: for each row, count beads and place them to the right (bottom is row 0)
    for (int r = 0; r < m; ++r) {
        int cnt = 0;
        for (int col = 0; col < n; ++col) if (beads[r][col]) ++cnt;
        // clear row
        for (int col = 0; col < n; ++col) beads[r][col] = 0;
        // put cnt beads on the right
        for (int k = 0; k < cnt; ++k) beads[r][n-1-k] = 1;
    }

    // Read column heights -> sorted ascending
    vector<int> res(n, 0);
    for (int col = 0; col < n; ++col)
        for (int r = 0; r < m; ++r)
            if (beads[r][col]) ++res[col];

    return res;
}

// Small test
int main() {
    vector<int> a = {5, 1, 4, 2};
    auto s = bead_sort_naive(a);
    for (int x : s) cout << x << " ";
    cout << "\n"; // expected: 1 2 4 5
    return 0;
}
```

**Notes:** clear educational code. Memory blows up if `m` or `n` is large.

---

## C++ — Optimized histogram/cumulative bead sort (practical)

```cpp
// bead_sort_optimized.cpp
// Practical bead sort using histogram/cumulative counts.
// Complexity: O(n + m + S) time, O(n + m) space. Works for negative values by shifting.

#include <bits/stdc++.h>
using namespace std;

// Sort ascending. Handles negative values by shifting.
vector<long long> bead_sort_optimized(const vector<long long>& arr_in) {
    int n = (int)arr_in.size();
    if (n == 0) return {};

    // Find min and max to possibly shift negatives
    long long minv = *min_element(arr_in.begin(), arr_in.end());
    long long shift = 0;
    if (minv < 0) shift = -minv;

    // Build shifted values and find max
    vector<long long> a(n);
    long long maxv = LLONG_MIN;
    long long S = 0; // sum of elements (after shift)
    for (int i = 0; i < n; ++i) {
        a[i] = arr_in[i] + shift;
        if (a[i] < 0) throw runtime_error("Overflow in shifting");
        maxv = max(maxv, a[i]);
        S += a[i];
    }

    if (maxv == 0) return vector<long long>(n, minv); // all identical (zero after shift)

    if (S > (long long)1e12) {
        // heuristic sanity check: if sum huge, bead sort may be impractical
        // but we won't stop — just warn by comment. In production, throw or fallback to counting/radix.
    }

    // histogram H[0..maxv]
    size_t m = (size_t)maxv;
    vector<long long> H(m + 1, 0);
    for (auto v : a) ++H[(size_t)v];

    // cumulative C[r] = number >= r  for r = 1..m
    vector<long long> C(m + 1, 0);
    long long cum = 0;
    for (long long r = (long long)m; r >= 1; --r) {
        cum += H[(size_t)r];
        C[(size_t)r] = cum;
    }

    // Build result heights: for each row r, increment the rightmost C[r] columns
    vector<long long> res(n, 0);
    for (size_t r = 1; r <= m; ++r) {
        long long cnt = C[r];
        // add 1 to res[n-1], res[n-2], ..., res[n-cnt]
        for (long long k = 0; k < cnt; ++k) {
            res[n - 1 - (size_t)k] += 1;
        }
    }

    // subtract shift to get original values
    for (int i = 0; i < n; ++i) res[i] -= shift;

    return res;
}

// Small test
int main() {
    vector<long long> a = {5, 1, 4, 2};
    auto s = bead_sort_optimized(a);
    for (auto x : s) cout << x << " ";
    cout << "\n"; // expected: 1 2 4 5

    vector<long long> b = {-1, 3, 0, -1};
    auto t = bead_sort_optimized(b);
    for (auto x : t) cout << x << " ";
    cout << "\n"; // expected: -1 -1 0 3
    return 0;
}
```

**Key points in this code**

* Uses `long long` to reduce overflow risk; you might require 128-bit counters for extreme sums.
* The inner double loop total iterations equal `S = sum(a)`. If `S` is massive, this will be slow.
* Works with negative values by shifting.

---

## Rust — Optimized histogram/cumulative bead sort

```rust
// bead_sort_optimized.rs
// Rust version: histogram + cumulative approach, ascending sort.

fn bead_sort_optimized(mut arr: Vec<i64>) -> Vec<i64> {
    if arr.is_empty() { return vec![]; }
    let n = arr.len();

    // shift if negative
    let &minv = arr.iter().min().unwrap();
    let shift = if minv < 0 { -minv } else { 0 };
    for v in &mut arr { *v += shift; }

    let maxv = *arr.iter().max().unwrap();
    if maxv == 0 {
        return vec![minv; n]; // all same
    }

    let m = maxv as usize;
    let mut hist = vec![0usize; m+1];
    let mut sum_beads: u128 = 0;
    for &v in &arr {
        hist[v as usize] += 1;
        sum_beads += v as u128;
    }

    // cumulative C[r] = number >= r
    let mut c = vec![0usize; m+1];
    let mut cum = 0usize;
    for r in (1..=m).rev() {
        cum += hist[r];
        c[r] = cum;
    }

    let mut res = vec![0i64; n];
    for r in 1..=m {
        let cnt = c[r];
        // add 1 to rightmost cnt columns
        for k in 0..cnt {
            let idx = n - 1 - k;
            res[idx] += 1;
        }
    }

    // subtract shift
    for x in &mut res { *x -= shift; }
    res
}

fn main() {
    let a = vec![5, 1, 4, 2];
    println!("{:?}", bead_sort_optimized(a)); // [1,2,4,5]

    let b = vec![-2, 0, 3, -2];
    println!("{:?}", bead_sort_optimized(b)); // [-2,-2,0,3]
}
```

**Rust notes**

* Uses `usize` for counts and `i64` for values.
* Watch `sum_beads` for overflow if values are huge; use larger integer types as needed.

---

# **Edge cases & tests to try**

* Empty array `[]` → `[]`
* All zeros `[0,0,0]` → `[0,0,0]`
* Negative numbers `[-5, -1, -3]` — requires shifting
* Very large values (e.g., `10^6`) — memory/time stress test: watch out
* Arrays with huge `S = sum(A)` (e.g., many large values) — optimized algorithm time = `O(S)` may be huge
* Single-element arrays `[x]` → `[x]`
* Duplicates `[2,2,2]` → `[2,2,2]` (not stable w\.r.t original indices, but stable in value terms)

---

# **Comparison with similar algorithms (concise)**

* **Counting sort:** Similar in spirit (non-comparison, uses counts). Complexity `O(n+k)`. Usually better than bead when range `k` is manageable.
* **Radix sort:** Works well for fixed-width integers; `O(n · d)` with small base `d`.
* **Pigeonhole sort:** Another histogram-like method; organizes by bucket per key.
* **Bead sort vs Counting sort:** bead sort’s cost depends on **sum of values** `S`, while counting sort depends on **range** `k`. For some data profiles bead may be worse; for others it may be competitive.
* **Parallel / hardware**: bead sort is naturally parallel (beads fall simultaneously) — so an actual physical/analog implementation may be very fast.

---

# **Advanced considerations**

* **Bitset packing**: Represent each row as machine words; use popcount and shifts to simulate gravity faster. This reduces memory and uses CPU bit-ops for speed.
* **SIMD/GPU**: Each row update (`C[r]`) updates rightmost `C[r]` columns — trivial to parallelize across rows or columns.
* **Streaming**: Hard to make purely streaming because bead sort needs global view (`C[r]` depends on all elements).
* **Proof-of-work / physical implementations**: People have implemented physical bead-sorting demonstrations — interesting pedagogical devices but not practical for production on large integers.

---

# **Recommendations & final checklist before using bead sort**

* ✅ Use only when data are non-negative integers or mappable to them.
* ✅ Compute `n`, `m`, and `S` first. If `m*n` or `S` is too large, **do not** use the naive approach!
* ✅ Prefer histogram/cumulative implementation for software.
* ✅ For extremely large ranges or sums, prefer counting/radix/quick/merge depending on constraints.
* ✅ Guard counters with 64/128-bit types where necessary.
* ✅ Test on edge inputs: empty, zeros, negatives, extremely large numbers.

---

