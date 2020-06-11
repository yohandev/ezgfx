mod vertex;

// -- re-export dependencies --
pub mod bytemuck { pub use bytemuck::*; }
pub mod wgpu { pub use wgpu::*; }

// -- export modules --
pub use vertex::{ Vertex, VertexAttribute };
pub use macros::*;