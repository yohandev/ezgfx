use spirv_reflect::types::{ ReflectDescriptorBinding, ReflectResourceType };
use image::*;
use wgpu::*;

use crate::{ Uniform, RenderQueue };

pub struct Texture
{
    view: TextureView
}

impl Uniform for Texture
{
    fn ty(&self) -> BindingType
    {
        BindingType::SampledTexture
        {
            multisampled: false,
            dimension: TextureViewDimension::D2,
            component_type: TextureComponentType::Uint
        }
    }

    fn resource(&self) -> BindingResource
    {
        BindingResource::TextureView(&self.view)
    }

    fn matches(&self, shader_repr: &ReflectDescriptorBinding) -> bool
    {
        shader_repr.resource_type == ReflectResourceType::ShaderResourceView
    }
}

impl Texture
{
    pub fn create(render: &RenderQueue, bytes: Box<[u8]>, name: Option<&str>) -> Self
    {
        let img =                                           // image
            image::load_from_memory(bytes.as_ref()).unwrap();
        let dim = img.dimensions();
        let rgba = img.into_rgba();

        let siz = Extent3d                                  // size
        { 
            width: dim.0,
            height: dim.1,
            depth: 1
        };
        let tex = render.device.create_texture              // texture usage
        (
            &TextureDescriptor
            {
                size: siz,
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
                label: name
            }
        );
        let buf = render.device.create_buffer_with_data     // texture buffer
        (
            &rgba,
            BufferUsage::COPY_SRC
        );

        let mut encoder =                                   // encoder
            render.device.create_command_encoder
            (
                &CommandEncoderDescriptor
                {
                    label: Some
                    (
                        format!("{}_texture_buffer_copy_encoder", name.unwrap_or("untitled"))
                            .as_str()
                    )
                }
            );

        encoder.copy_buffer_to_texture                      // copy buffer
        (
            BufferCopyView
            {
                buffer: &buf,
                offset: 0,
                bytes_per_row: 4 * dim.0,
                rows_per_image: dim.1
            },
            TextureCopyView
            {
                texture: &tex,
                mip_level: 0,
                array_layer: 0,
                origin: Origin3d::ZERO
            },
            siz
        );

        let cmd = encoder.finish();                         // encoder command, later sent to queue

        let view = tex.create_default_view();               // texture view

        render.queue.submit(&[ cmd ]);                      // copy tex buffer to tex view

        Self { view }
    }

    pub fn from_file(render: &RenderQueue, path: &str) -> Self
    {
        let rpath = std::path::Path::new // fallback: std::env::current_dir().unwrap().join(path)
        (
            std::env::var("CARGO_MANIFEST_DIR")
                .unwrap()
                .as_str()
        )
        .join("src")
        .join(path);

        let bytes = std::fs::read(&rpath)
            .expect(format!("texture not found: {:?}", rpath).as_str());

        let name = rpath.file_name().unwrap().to_str();

        Self::create(render, bytes.into_boxed_slice(), name)
    }
}