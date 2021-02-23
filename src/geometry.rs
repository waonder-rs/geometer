use crate::{
	vertex,
	Vertex
};

/// Triangular face.
#[derive(Clone, Copy)]
pub struct Face {
	pub a: u32,
	pub b: u32,
	pub c: u32
}

impl Face {
	pub fn new(a: u32, b: u32, c: u32) -> Face {
		Face {
			a, b, c
		}
	}

	pub fn max_index(&self) -> u32 {
		std::cmp::max(self.a, std::cmp::max(self.b, self.c))
	}
}

/// Sets of faces at some precision.
#[derive(Clone)]
pub struct Precision {
	faces: Vec<Face>,
	max_index: u32
}

impl Precision {
	pub fn new() -> Precision {
		Precision {
			faces: Vec::new(),
			max_index: 0
		}
	}

	pub fn max_index(&self) -> u32 {
		self.max_index
	}

	pub fn add(&mut self, face: Face) {
		self.max_index = std::cmp::max(self.max_index, face.max_index());
		self.faces.push(face)
	}
}

/// Typed geometry.
pub struct Geometry<V: Vertex> {
	/// Typed vertices.
	vertices: Vec<V>,

	/// Sets of faces for each precision.
	/// 
	/// A geometry can come with multiple level of precision.
	precisions: Vec<Precision>
}

impl<V: Vertex> Geometry<V> {
	pub fn new(vertices: Vec<V>) -> Geometry<V> {
		Geometry {
			vertices,
			precisions: Vec::new()
		}
	}

	pub fn add_precision(&mut self, p: Precision) {
		assert!((p.max_index as usize) < self.vertices.len());
		self.precisions.push(p)
	}
}

/// Untyped geometry.
pub struct AbstractGeometry {
	/// Layout of vertices.
	layout: vertex::Layout,

	/// Raw vertices.
	vertices: Vec<u8>,

	/// Sets of faces for each precision.
	precisions: Vec<Precision>
}

impl AbstractGeometry {
	pub fn layout(&self) -> vertex::Layout {
		self.layout
	}

	pub fn vertices(&self) -> &[u8] {
		&self.vertices
	}

	pub fn precisions(&self) -> &[Precision] {
		&self.precisions
	}
}

impl<V: Vertex> From<Geometry<V>> for AbstractGeometry {
	fn from(g: Geometry<V>) -> AbstractGeometry {
		AbstractGeometry {
			layout: V::LAYOUT,
			vertices: unsafe { std::mem::transmute(g.vertices) },
			precisions: g.precisions
		}
	}
}