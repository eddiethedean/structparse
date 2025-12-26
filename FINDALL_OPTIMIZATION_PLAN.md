# Findall Advanced Optimization Plan: Threading & Vectorization

## Current Bottleneck Analysis

The main bottleneck in `findall` is **Python object creation overhead**:
- Each match calls `convert_value()` which creates `PyObject` (requires GIL)
- Each match creates a `ParseResult` Python object via `Py::new()` (FFI overhead)
- With 10 matches, that's 10+ FFI boundary crossings

## Optimization Strategy

### 1. Defer Python Object Creation (Highest Impact)

**Problem**: We create Python objects (`PyObject`) during the matching loop, requiring GIL for each conversion.

**Solution**: 
- Collect raw data first (strings, numbers as Rust types)
- Batch create all Python objects at the end in a single GIL block
- This reduces GIL overhead from N operations to 1 operation

**Implementation**:
- Create a `RawMatchData` struct to hold extracted values as Rust types
- Modify `match_with_captures` to return `RawMatchData` instead of `PyObject`
- Batch convert all `RawMatchData` to Python objects at once

**Expected Impact**: 30-50% improvement

### 2. Parallel Regex Matching (Medium Impact)

**Problem**: Regex matching is CPU-bound but sequential.

**Solution**:
- Use Rust threads (rayon) to find all matches in parallel
- Split string into chunks and match in parallel
- Collect results, then batch create Python objects

**Challenges**:
- Need to handle overlapping matches across chunk boundaries
- Regex captures_iter is already efficient, but we can parallelize the processing

**Expected Impact**: 20-40% improvement (on multi-core systems)

### 3. SIMD-Optimized Type Conversion (Medium Impact)

**Problem**: Type conversions (parsing integers, floats) are done sequentially.

**Solution**:
- Use SIMD for batch parsing of numbers
- Libraries like `lexical-core` or custom SIMD implementations
- Parse multiple numbers at once using vector instructions

**Expected Impact**: 10-20% improvement (for numeric patterns)

### 4. Object Pooling (Low Impact)

**Problem**: Creating new Python objects has allocation overhead.

**Solution**:
- Pre-allocate a pool of `ParseResult` objects
- Reuse objects from the pool
- Only create new ones when pool is exhausted

**Expected Impact**: 5-10% improvement

## Recommended Implementation Order

### Phase 1: Defer Python Object Creation (Highest ROI)
1. Create `RawMatchData` struct
2. Modify `match_with_captures` to return `RawMatchData`
3. Batch convert all results at the end
4. **Expected**: 30-50% improvement

### Phase 2: Parallel Processing (If Phase 1 isn't enough)
1. Add `rayon` dependency
2. Parallelize regex matching (if string is long enough)
3. Collect and merge results
4. **Expected**: Additional 20-40% improvement

### Phase 3: SIMD Optimizations (If still needed)
1. Research SIMD libraries for number parsing
2. Implement batch number parsing
3. **Expected**: Additional 10-20% improvement

## Code Structure Changes

### New Types Needed

```rust
// Raw match data (no Python objects)
struct RawMatchData {
    fixed: Vec<RawValue>,  // Rust types, not PyObject
    named: HashMap<String, RawValue>,
    span: (usize, usize),
    field_spans: HashMap<String, (usize, usize)>,
}

enum RawValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    // ... other types
}
```

### Modified Function Signature

```rust
// Instead of returning PyObject immediately:
fn match_with_captures(...) -> PyResult<Option<PyObject>>

// Return raw data:
fn match_with_captures_raw(...) -> PyResult<Option<RawMatchData>>

// Then batch convert:
fn batch_convert_to_python(py: Python, raw_data: Vec<RawMatchData>) -> PyResult<PyList>
```

## Threading Considerations

### GIL Handling
- Rust threads can do regex matching and data extraction WITHOUT GIL
- Only need GIL when creating Python objects
- Batch all Python object creation in a single GIL block

### Thread Safety
- Regex `Captures` are not `Send`, need to extract data before sending to threads
- String slices are safe to share (read-only)
- Use `Arc` for shared parser data

## Vectorization Opportunities

### Number Parsing
- Parse multiple integers/floats at once using SIMD
- Libraries: `lexical-core`, `fast-float`, or custom SIMD
- Only beneficial for patterns with many numeric fields

### String Operations
- SIMD string comparisons
- SIMD character classification
- Less impactful since regex crate may already optimize

## Success Criteria

- `findall` with 10 matches: Should be **faster** than original parse library
- Target: 1.2-1.5x **faster** (currently 1.2-1.5x slower)
- All tests should pass
- No API changes

## Risk Assessment

### Low Risk
- Deferring Python object creation (Phase 1)
- Well-tested pattern, minimal code changes

### Medium Risk  
- Parallel processing (Phase 2)
- Need to handle edge cases (overlapping matches, thread safety)
- May not help on single-core systems

### Higher Risk
- SIMD optimizations (Phase 3)
- Platform-specific code
- May require unsafe Rust
- Diminishing returns

## Recommendation

**Start with Phase 1** (Defer Python Object Creation):
- Highest impact (30-50% improvement expected)
- Lowest risk
- Minimal code changes
- Should be sufficient to make findall faster than original

If Phase 1 isn't enough, proceed to Phase 2 (Parallel Processing).

