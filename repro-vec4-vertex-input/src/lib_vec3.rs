#![no_std]

use glam::{Vec3, Vec4};
use spirv_std::spirv;

#[spirv(vertex)]
pub fn main_vs(in_pos: Vec3, #[spirv(position)] out_position: &mut Vec4) {
    *out_position = in_pos.extend(1.0);
}
