#![no_std]

use glam::Vec4;
use spirv_std::spirv;

// BUG: `in_pos: Vec4` silently drops the vertex entry point.
// Changing Vec4 -> Vec3 (glam) emits `main_vs` correctly.
#[spirv(vertex)]
pub fn main_vs(in_pos: Vec4, #[spirv(position)] out_position: &mut Vec4) {
    *out_position = in_pos;
}
