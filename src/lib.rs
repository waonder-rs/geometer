#![feature(const_ptr_offset_from, const_maybe_uninit_as_ptr, const_raw_ptr_deref)]

pub mod vertex;
pub mod geometry;

pub use vertex::Vertex;
pub use geometry::{
	Geometry,
	AbstractGeometry
};