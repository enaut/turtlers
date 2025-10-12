# turtle-lib-macroquad

The main turtle graphics library built on Macroquad with Lyon tessellation.

**See the [main README](../README.md) for complete documentation.**

## Features

✅ **Complete Lyon Integration** - All drawing operations use GPU-optimized tessellation
- Unified rendering pipeline for lines, arcs, circles, and fills
- ~410 lines of code eliminated through architectural simplification
- Consistent high-quality rendering across all primitives

✅ **Multi-Contour Fill System** - Advanced fill capabilities with automatic hole detection
- EvenOdd fill rule for complex shapes with holes (like cheese or yin-yang symbols)
- `pen_up()` closes current contour, `pen_down()` opens next contour
- Progressive fill preview during animations
- Support for self-intersecting paths

✅ **Smooth Animation** - Tweening system with live rendering
- Configurable speed control
- Frame-rate independent animation
- Live fill preview during circle/arc drawing

## Quick Examples

```bash
# Run from this directory
cargo run --example square          # Basic square drawing
cargo run --example yinyang         # Multi-contour fills with holes
cargo run --example fill_advanced   # Self-intersecting fills
cargo run --example koch            # Recursive fractals
cargo run --example fill_demo       # Multiple independent fills
```

## Architecture Highlights

### Rendering Pipeline
All drawing operations → Lyon tessellation → GPU mesh rendering

### DrawCommand Enum
Simplified from 5 variants to 1:
- `Mesh(MeshData)` - unified variant for all drawing operations

### Fill System
- `FillState` tracks multiple contours (completed + current)
- Pen state management automatically handles contour creation
- EvenOdd tessellation provides automatic hole detection

See [LYON_COMPLETE.md](LYON_COMPLETE.md) and [MULTI_CONTOUR_FILLS.md](MULTI_CONTOUR_FILLS.md) for implementation details.

## Status

✅ **Stable** - Lyon integration complete, multi-contour fills working, all examples passing.

See [../README.md](../README.md) for full API documentation and project status.
