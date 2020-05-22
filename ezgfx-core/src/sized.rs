use bytemuck::*;

pub trait BufMember: Pod
{
    /// size of a single vertex, in bytes
    const SIZE: usize;
}

impl BufMember for [f32; 4] { const SIZE: usize = std::mem::size_of::<[f32; 4]>(); }
impl BufMember for [f32; 3] { const SIZE: usize = std::mem::size_of::<[f32; 3]>(); }
impl BufMember for [f32; 2] { const SIZE: usize = std::mem::size_of::<[f32; 2]>(); }
impl BufMember for [f32; 1] { const SIZE: usize = std::mem::size_of::<[f32; 1]>(); }
impl BufMember for f32 { const SIZE: usize = std::mem::size_of::<f32>(); }

impl BufMember for [i32; 4] { const SIZE: usize = std::mem::size_of::<[i32; 4]>(); }
impl BufMember for [i32; 3] { const SIZE: usize = std::mem::size_of::<[i32; 3]>(); }
impl BufMember for [i32; 2] { const SIZE: usize = std::mem::size_of::<[i32; 2]>(); }
impl BufMember for [i32; 1] { const SIZE: usize = std::mem::size_of::<[i32; 1]>(); }
impl BufMember for i32 { const SIZE: usize = std::mem::size_of::<i32>(); }

impl BufMember for [u32; 4] { const SIZE: usize = std::mem::size_of::<[u32; 4]>(); }
impl BufMember for [u32; 3] { const SIZE: usize = std::mem::size_of::<[u32; 3]>(); }
impl BufMember for [u32; 2] { const SIZE: usize = std::mem::size_of::<[u32; 2]>(); }
impl BufMember for [u32; 1] { const SIZE: usize = std::mem::size_of::<[u32; 1]>(); }
impl BufMember for u32 { const SIZE: usize = std::mem::size_of::<u32>(); }