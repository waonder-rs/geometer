mod position;
mod normal;

pub use position::*;
pub use normal::*;

/// Vertex position format.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PositionFormat {
	F32XYZ
}

/// Vertex layout.
/// 
/// Describes how to access the data of each vertex
/// in a geometry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Layout {
	/// Size of one vertex data.
	stride: usize,

	/// Position of the vertex.
	/// 
	/// A vertex must have a position.
	position: (usize, PositionFormat),

	/// Normal of the vertex, if any.
	normal: Option<(usize, PositionFormat)>
}

/// Mesh vertex.
pub unsafe trait Vertex: Copy {
	const LAYOUT: Layout;
}