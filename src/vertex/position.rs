use glam::Vec3;
use crate::{
	vertex,
	Vertex
};

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Position3(pub Vec3);

impl Position3 {
	#[inline]
	pub fn new(x: f32, y: f32, z: f32) -> Position3 {
		Position3(Vec3::new(x, y, z))
	}
}

unsafe impl Vertex for Position3 {
	const LAYOUT: vertex::Layout = vertex::Layout {
		stride: std::mem::size_of::<Self>(),
		position: (0, vertex::PositionFormat::F32XYZ),
		normal: None
	};
}