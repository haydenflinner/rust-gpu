# `#[spirv(vertex)]` with a `Vec4` vertex input silently emits no `OpEntryPoint`

## Summary

A `#[spirv(vertex)]` entry point that takes a `glam::Vec4` vertex attribute as
its position input is **silently dropped** from the compiled SPIR-V module:
the resulting `.spv` contains zero `OpEntryPoint` instructions. The identical
function using `glam::Vec3` emits the entry point correctly.

Under `--multimodule` builds (the `cargo gpu build` default) the failure is
invisible — no warning is printed and the `.spv` for that module is simply
missing its vertex entry point. In single-module builds `spirv-val` fails:

```
error: No OpEntryPoint instruction was found. This is only allowed if the
Linkage capability is being used.
```

## Minimal repro

`repro-vec4-vertex-input/` is a standalone shader crate:

```rust
#![no_std]

use glam::Vec4;
use spirv_std::spirv;

#[spirv(vertex)]
pub fn main_vs(in_pos: Vec4, #[spirv(position)] out_position: &mut Vec4) {
    *out_position = in_pos;
}
```

Build it:

```sh
cargo gpu build --shader-crate repro-vec4-vertex-input \
    --output-dir out --auto-install-rust-toolchain
```

**Expected:** `repro_vec4_vertex_input.spv` contains
`OpEntryPoint Vertex %main_vs`.

**Actual:** module compiles with no error; disassembling shows **zero**
`OpEntryPoint` instructions and no `main_vs` symbol. `spirv-val` rejects the
module for missing `OpEntryPoint`.

## Control case

`src/lib_vec3.rs` is the same function with `in_pos: Vec3`
(`*out_position = in_pos.extend(1.0)`). Swapping it in for `src/lib.rs` and
rebuilding produces a valid module with
`OpEntryPoint Vertex %main_vs` present.

## Environment where observed

- `spirv-std` @ `877bd8697a15f3e6d09446a5e1807e6237ca1dac` (the rev pinned by
  `crates/shader-crate-template`), `glam 0.30.9`, `default-features = false`
- nightly-2026-03-14 (the toolchain `REQUIRED_RUST_TOOLCHAIN` resolves to at
  that rev)
- `cargo-gpu` installed from the archived `Rust-GPU/cargo-gpu` repo

## Real-world instance

This bites in `Rust-GPU/VulkanShaderExamples`:
`shaders/rust/deferredshadows/shadow/src/lib.rs` declares

```rust
#[spirv(vertex)]
pub fn main_vs(in_pos: Vec4, #[spirv(instance_index)] instance_index: u32, ...)
```

and the compiled `shadow.vert.spv` ships with no vertex entry point, so the
example fails at pipeline creation. Found while building a GLSL-vs-Rust
difftest harness across the repo's 77 examples — this was the only shader in
the corpus that silently produced a broken module.

## Suspected cause

`Vec4` vertex attributes likely need to be split into two `Location`s (or the
attribute-lowering path rejects a 4-wide input and the drop isn't surfaced).
Either way, silently dropping the entry point is the bug — at minimum it
should be a compile error.
