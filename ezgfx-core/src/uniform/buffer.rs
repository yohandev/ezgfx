use crate::Uniform;

pub trait BufferData: bytemuck::Pod
{
    const SIZE: usize;
}

pub struct Buffer<T: BufferData>
{
    pub data: T,
    
    buf: wgpu::Buffer
}

impl<T: BufferData> Uniform for Buffer<T>
{
    fn ty(&self) -> wgpu::BindingType
    {
        wgpu::BindingType::UniformBuffer { dynamic: false }
    }

    fn resource(&self) -> wgpu::BindingResource
    {
        wgpu::BindingResource::Buffer
        {
            buffer: &self.buf,
            range: 0..T::SIZE as wgpu::BufferAddress
        }
    }
}

impl<T: BufferData> Buffer<T>
{
    pub fn create(render: &crate::RenderQueue, data: T) -> Self
    {
        let buf = render.device.create_buffer_with_data
        (
            bytemuck::cast_slice(&[data]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        );

        Self
        {
            data,
            buf
        }
    }
}