use memoffset::offset_of_tuple;
use glam::Vec3;
use crate::{
	vertex,
	Vertex
};
use super::Position3;

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Normal3(Vec3);

unsafe impl Vertex for (Position3, Normal3) {
	const LAYOUT: vertex::Layout = vertex::Layout {
		stride: std::mem::size_of::<Self>(),
		position: (offset_of_tuple!(Self, 0), vertex::PositionFormat::F32XYZ),
		normal: Some((offset_of_tuple!(Self, 1), vertex::PositionFormat::F32XYZ))
	};
}