// use image::*;
// use wgpu::*;

// pub struct TextureSet;

// impl TextureSet
// {
//     pub fn load(files: &[&str])
//     {
//         let textures = files                                // multiple textures
//             .iter()
//             .map(|f| Self::load_img(f));
        
//         let sampler = device.create_sampler                 // one texture sampler
//         (
//             &SamplerDescriptor
//             {
//                 address_mode_u: AddressMode::Repeat,
//                 address_mode_v: AddressMode::Repeat,
//                 address_mode_w: AddressMode::Repeat,
//                 mag_filter: FilterMode::Linear, // TODO change to nearest for pixel textures
//                 min_filter: FilterMode::Nearest,
//                 mipmap_filter: FilterMode::Nearest,
//                 lod_min_clamp: -100.0,
//                 lod_max_clamp: 100.0,
//                 compare: CompareFunction::Always
//             }
//         );

//         let b_entries = vec![];                             // binding entries
//         let bl_entries = vec![];                            // binding layout entries
//         for tex in textures
//         {
//             b_entries.push
//             (
//                 Binding
//                 {
//                     binding: b_entries.len() as u32,
//                     resource: BindingResource::TextureView(&tex.0)
//                 }
//             );
//             bl_entries.push
//             (
//                 BindGroupLayoutEntry
//                 {
//                     binding: bl_entries.len() as u32,
//                     visibility: ShaderStage::FRAGMENT,
//                     ty: BindingType::SampledTexture
//                     {
//                         multisampled: false,
//                         dimension: TextureViewDimension::D2,
//                         component_type: TextureComponentType::Uint
//                     }
//                 }
//             )
//         }
//         b_entries.push
//         (
//             Binding
//             {
//                 binding: b_entries.len() as u32,
//                 resource: BindingResource::Sampler(&sampler)
//             }
//         );
//         bl_entries.push
//         (
//             BindGroupLayoutEntry
//             {
//                 binding: 1,
//                 visibility: ShaderStage::FRAGMENT,
//                 ty: BindingType::Sampler
//                 {
//                     comparison: false
//                 }
//             }
//         );

//         let bind_layout = device.create_bind_group_layout   // bind group layout
//         (
//             &BindGroupLayoutDescriptor
//             {
//                 bindings: bl_entries.as_slice(),
//                 label: Some("texture_bind_group_layout")
//             }
//         );
//         let bind = device.create_bind_group                 // bind group
//         (
//             &BindGroupDescriptor
//             {
//                 layout: &bind_layout,
//                 bindings: b_entries.as_slice(),
//                 label: Some("texture_bind_group")
//             }
//         );

//         for tex in textures
//         {
//             queue.submit(&[ tex.1 ]);
//         }
//     }

//     fn load_img(path: &str) -> (TextureView, CommandEncoder)
//     {
//         let img = image::open(path).unwrap();
//         let dim = img.dimensions();
//         let rgba = img.into_rgba();

//         let siz = Extent3d                                  // size
//         { 
//             width: dim.0,
//             height: dim.1,
//             depth: 1
//         };
//         let tex = device.create_texture                     // texture usage
//         (
//             &TextureDescriptor
//             {
//                 size: siz,
//                 array_layer_count: 1,
//                 mip_level_count: 1,
//                 sample_count: 1,
//                 dimension: TextureDimension::D2,
//                 format: TextureFormat::Rgba8UnormSrgb,
//                 usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
//                 label: Some(path)
//             }
//         );

//         let buf = device.create_buffer_with_data            // texture buffer
//         (
//             &rgba,
//             BufferUsage::COPY_SRC
//         );

//         let mut encoder = device.create_command_encoder     // encoder
//         (
//             &CommandEncoderDescriptor
//             {
//                 label: Some("texture_buffer_copy_encoder")
//             }
//         );

//         encoder.copy_buffer_to_texture                      // copy buffer
//         (
//             BufferCopyView
//             {
//                 buffer: &buf,
//                 offset: 0,
//                 bytes_per_row: 4 * dim.0,
//                 rows_per_image: dim.1
//             },
//             TextureCopyView
//             {
//                 texture: &tex,
//                 mip_level: 0,
//                 array_layer: 0,
//                 origin: Origin3d::ZERO
//             },
//             siz
//         );

//         let cmd = encoder.finish();                         // encoder command, later sent to queue
//         let view = tex.create_default_view();               // texture view
        
//         (view, cmd)
//     }
// }