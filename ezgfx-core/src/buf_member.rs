use bytemuck::*;

/// a raw-memory, sized, member of a buffer
pub trait BufMember: Pod
{
    /// size of a single buffer member, in bytes
    const SIZE: usize;
}

macro_rules! impl_buf_member
{
    ( $ty:ident , $( $n:expr ),* ) =>
    {
        $(impl BufMember for [$ty; $n] { const SIZE: usize = std::mem::size_of::<[$ty; $n]>(); })*
    }
}

impl_buf_member!(f64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096);
impl_buf_member!(f32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096);
impl_buf_member!(i32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096);
impl_buf_member!(u32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096);

impl BufMember for f64 { const SIZE: usize = std::mem::size_of::<f64>(); }
impl BufMember for f32 { const SIZE: usize = std::mem::size_of::<f32>(); }
impl BufMember for i32 { const SIZE: usize = std::mem::size_of::<i32>(); }
impl BufMember for u32 { const SIZE: usize = std::mem::size_of::<u32>(); }