use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn impl_pipeline(args: TokenStream, item: TokenStream) -> TokenStream
{
    // parse item
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    // dissect input
    let name = &input.ident;
    let vis = &input.vis;
    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();
    let s_paths = parse_args(&args);

    // assert enough shaders
    assert!(s_paths.len() == 2, "expected 2(vert, frag) shader paths!");

    // determine type
    let (ver_ty, ind_ty, rem, rem_ty) = find_buffer_types(&input);
    let bindings = 0..rem_ty.len();

    // impl trait
    let expanded = quote!
    {
        #vis struct #name #type_gene #where_clause
        {
            #(#rem),*
        }

        impl #impl_gene ezgfx::Pipeline for #name #type_gene #where_clause
        {
            const TYPE: ezgfx::PipelineType = ezgfx::PipelineType::Render;

            const SHADER_PATHS: &'static [&'static str] =
            &[
                #(#s_paths),*
            ];

            fn create(queue: ezgfx::RenderQueue)
            {
                let i = ezgfx::wgpu::VertexStateDescriptor
                {
                    index_format: <#ind_ty as ezgfx::Index>::FORMAT,
                    vertex_buffers:
                    &[
                        ezgfx::wgpu::VertexBufferDescriptor
                        {
                            stride: <#ver_ty as ezgfx::Vertex>::SIZE as ezgfx::wgpu::BufferAddress,
                            step_mode: ezgfx::wgpu::InputStepMode::Vertex,
                            attributes: <#ver_ty as ezgfx::Vertex>::DESC
                        }
                    ]
                };

                // let pip_layout = queue.device             // pipeline layout
                //     .create_pipeline_layout
                //     (
                //         &ezgfx::wgpu::PipelineLayoutDescriptor
                //         {
                //             bind_group_layouts:
                //             &[
                //                 #(&< #rem_ty as ezgfx::PipelineResource>::bind_layout( #bindings )),*
                //             ]
                //         }
                //     );

                // let pipeline = device               // pipeline
                //     .create_render_pipeline
                //     (
                //         &ezgfx::wgpu::RenderPipelineDescriptor
                //         {
                //             layout: &pip_layout,
                //             vertex_stage: ezgfx::wgpu::ProgrammableStageDescriptor
                //             {
                //                 module: &shader.modules[0],
                //                 entry_point: "main"
                //             },
                //             fragment_stage: Some(ezgfx::wgpu::ProgrammableStageDescriptor
                //             {
                //                 module: &shader.modules[1],
                //                 entry_point: "main"
                //             }),
                //             rasterization_state: Some(ezgfx::wgpu::RasterizationStateDescriptor
                //             {
                //                 front_face: ezgfx::wgpu::FrontFace::Ccw,
                //                 cull_mode: ezgfx::wgpu::CullMode::Back,
                //                 depth_bias: 0,
                //                 depth_bias_slope_scale: 0.0,
                //                 depth_bias_clamp: 0.0
                //             }),
                //             color_states: 
                //             &[
                //                 ezgfx::wgpu::ColorStateDescriptor
                //                 {
                //                     format: format,
                //                     color_blend: ezgfx::wgpu::BlendDescriptor::REPLACE,
                //                     alpha_blend: ezgfx::wgpu::BlendDescriptor::REPLACE,
                //                     write_mask: ezgfx::wgpu::ColorWrite::ALL
                //                 }
                //             ],
                //             primitive_topology: ezgfx::wgpu::PrimitiveTopology::TriangleList,
                //             depth_stencil_state: None,
                //             vertex_state: ezgfx::wgpu::VertexStateDescriptor
                //             {
                //                 index_format: I::format(),
                //                 vertex_buffers:
                //                 &[
                //                     ezgfx::wgpu::VertexBufferDescriptor
                //                     {
                //                         stride: mem::size_of::<V>() as ezgfx::wgpu::BufferAddress,
                //                         step_mode: ezgfx::wgpu::InputStepMode::Vertex,
                //                         attributes: V::desc()
                //                     }
                //                 ]
                //             },
                //             sample_count: 1,
                //             sample_mask: !0,
                //             alpha_to_coverage_enabled: false
                //         }
                //     );
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_args(args: &Vec<NestedMeta>) -> Vec<String>
{
    args.iter().map
    (
        |a| match a
        {
            NestedMeta::Lit(b) => match b
            {
                Lit::Str(c) => c.value(),
                _ => panic!("expected string path to shader(s)!")
            },
            _ => panic!("expected string path to shader(s)!")
        }
    )
    .collect()
}

fn find_buffer_types(data: &ItemStruct) -> (&Type, &Type, Vec<&Field>, Vec<&Type>)
{
    let fields = match &data.fields
    {
        Fields::Named(fields) => &fields.named,
        _ => panic!("unnamed and unit structs aren't supported!"),
    };

    let mut ver_ty = None;      // vertex type
    let mut ind_ty = None;      // index type
    let mut rem = vec![];       // remaining fields
    let mut rem_ty = vec![];    // remaining field types

    for field in fields
    {
        if let Some(name) = &field.ident
        {
            if name == "VERTEX" || name == "vertex" || name == "VERT" || name == "vert"
            {
                ver_ty = Some(&field.ty);
            }
            else if name == "INDEX" || name == "index" || name == "IND" || name == "ind"
            {
                ind_ty = Some(&field.ty);
            }
            else
            {
                rem.push(field);
                rem_ty.push(&field.ty);
            }
        }
    }

    assert!(ver_ty.is_some(), "render pipeline needs to define \"VERTEX\" field along with the type of vertex it uses!");
    assert!(ind_ty.is_some(), "render pipeline needs to define \"INDEX\" field along with the type of index it uses!");

    (ver_ty.unwrap(), ind_ty.unwrap(), rem, rem_ty)
}

/// #[attribute_attr(set = 0, stage = vertex)] --> #[attribute_attr(0, vertex)]
fn pack_pipeline_attr(args: TokenStream, item: TokenStream) -> TokenStream
{
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    
}