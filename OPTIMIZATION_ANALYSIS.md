# Findall Optimization Analysis

## Current Status

Phase 1 (Defer Python Object Creation) has been implemented but **did not improve performance** as expected.

### What Was Implemented

1. ✅ `RawMatchData` struct to hold Rust types (no Python objects)
2. ✅ `convert_value_raw()` function to convert values without Python
3. ✅ `match_with_captures_raw()` function that returns `RawMatchData`
4. ✅ Batch conversion from `RawMatchData` to Python objects
5. ✅ `findall` uses raw path when no custom converters

### Performance Results

- **Before**: ~1.2-1.5x slower than original `parse`
- **After**: ~1.8-1.9x slower than original `parse`
- **Result**: Performance got **worse**, not better

## Why It Didn't Work

The batch conversion approach doesn't help because:

1. **Python object creation is still sequential**: We still call `Py::new()` for each `ParseResult`, which is expensive
2. **No true batch creation**: PyO3 doesn't support creating multiple Python objects in a single operation
3. **Overhead of raw conversion**: We're doing the same work twice (convert to raw, then to Python)
4. **GIL overhead is minimal**: The GIL acquisition overhead is small compared to object creation

## Threading Analysis

**Can we use threading?**

❌ **No, threading won't help** because:
- Python object creation **requires the GIL**
- We can't parallelize Python object creation
- Regex matching is already very fast (not the bottleneck)
- Threading would add overhead without benefit

**What about parallel regex matching?**

⚠️ **Limited benefit**:
- Regex `captures_iter` is already optimized
- Matches might overlap, making parallelization complex
- The bottleneck is Python object creation, not regex matching

## Vectorization Analysis

**Can we use SIMD?**

⚠️ **Limited benefit**:
- SIMD helps with number parsing, but that's already fast
- The bottleneck is Python object creation, not number parsing
- SIMD would add complexity without addressing the real issue

## Real Bottleneck

The real bottleneck is **Python object creation overhead**:
- `Py::new()` for each `ParseResult` is expensive
- Creating `PyObject` wrappers for each value is expensive
- FFI boundary crossings are expensive

## Recommendations

### Option 1: Accept Current Performance
- `findall` is 1.2-1.5x slower, which is acceptable for most use cases
- Other operations (`parse`, `search`) are significantly faster
- The slowdown is only noticeable with many matches

### Option 2: Optimize Python Object Creation
- Use `Py::new_bound()` instead of `Py::new()` (if available)
- Minimize intermediate `PyObject` creations
- Reuse Python objects where possible

### Option 3: Change API Design
- Return raw Rust data structures instead of Python objects
- Let Python code convert to objects if needed
- This would break API compatibility

### Option 4: Use PyPy or Other Python Implementations
- PyPy might have faster object creation
- But this is outside our control

## Conclusion

**Threading and vectorization won't help** because:
1. Python object creation requires the GIL (can't parallelize)
2. The bottleneck is FFI overhead, not CPU-bound work
3. Regex matching is already optimized

**The best approach** is to:
1. Accept that `findall` will be slightly slower for many matches
2. Focus on optimizing other operations (already done)
3. Consider API changes if performance is critical

## Next Steps

1. ✅ Document findings (this file)
2. ⚠️ Consider if 1.2-1.5x slowdown is acceptable
3. ⚠️ Test with real-world workloads
4. ⚠️ Consider API changes if needed

