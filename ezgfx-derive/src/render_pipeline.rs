use proc_macro::TokenStream;
use inflector::Inflector;
use proc_macro2::Span;
use quote::quote;
use shaderc::*;
use syn::*;

use crate::util::*;

pub fn process(item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(item as ItemImpl);

    let v_type = get_type_value("Vertex", &input.items);
    let i_type = get_type_value("Index", &input.items);

    let v_path = match get_const_value("VERT_PATH", "str", &input.items)
    {
        Lit::Str(s) => s.value(),
        _ => panic!("an error occured...") 
    };
    let f_path = match get_const_value("FRAG_PATH", "str", &input.items)
    {
        Lit::Str(s) => s.value(),
        _ => panic!("an error occured...") 
    };

    if input.items.len() > 4
    {
        panic!("additional fields not related to the render pipeline will be deleted. move them to an external impl block");
    }

    let vert = compile(ShaderKind::Vertex, v_path.as_str());
    let frag = compile(ShaderKind::Fragment, f_path.as_str());

    let v = vert.as_binary_u8();
    let f = frag.as_binary_u8();

    let vu = uniforms(&v);
    let fu = uniforms(&f);

    let (binding_group_layouts, binding_groups, sets) = bind_group_layout_descriptors(&[&vu, &fu]);
    let binding_group_layout_sets = sets.iter().map(|s| Ident::new(format!("binding_group_layout_set_{}", s).as_str(), Span::call_site()));
    let binding_group_layout_sets2 = binding_group_layout_sets.clone();
    let binding_group_sets = sets.iter().map(|s| Ident::new(format!("binding_group_set_{}", s).as_str(), Span::call_site()));
    let binding_group_layout_sets_ref = sets.iter().map(|s| Ident::new(format!("binding_group_layout_set_{}", s).as_str(), Span::call_site()));

    let uniform_names = vu.iter().chain(fu.iter()).map(|u| &u.name);

    let pipeline_name = input.self_ty;
    let pipeline_name_str = quote! {#pipeline_name}.to_string().to_snake_case();

    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();

    let expanded = quote!
    {
        impl #impl_gene #pipeline_name #type_gene #where_clause
        {
            pub fn create(ctx: &ezgfx::RenderContext, #(#uniform_names : &ezgfx::Uniform,)*) -> ezgfx::wgpu::RenderPipeline
            {
                // -- create bind group layouts per set --
                #(
                    let #binding_group_layout_sets = ctx.device.create_bind_group_layout
                    (
                        &ezgfx::wgpu::BindGroupLayoutDescriptor
                        {
                            bindings:
                            &[
                                #binding_group_layouts
                            ],
                            label: Some(format!("{}_bind_group_layout", #pipeline_name_str).as_str())
                        }
                    );
                )*

                // -- create bind groups per set --
                #(
                    let #binding_group_sets = ctx.device.create_bind_group
                    (
                        &ezgfx::wgpu::BindGroupDescriptor
                        {
                            layout: &#binding_group_layout_sets_ref,
                            bindings:
                            &[
                                #binding_groups
                            ],
                            label: Some(format!("{}_bind_group", #pipeline_name_str).as_str())
                        }
                    );
                )*

                // -- create pipeline layout --
                let pip_layout = ctx.device
                    .create_pipeline_layout
                    (
                        &ezgfx::wgpu::PipelineLayoutDescriptor
                        {
                            bind_group_layouts: &[#(&#binding_group_layout_sets2),*]
                        }
                    );
                
                // -- create shader modules --
                let (v, f) =
                {
                    const V_SPIRV: &[u8] = &[#(#v,)*];
                    const F_SPIRV: &[u8] = &[#(#f,)*];

                    let v = ezgfx::wgpu::read_spirv(std::io::Cursor::new(&V_SPIRV[..])).unwrap();
                    let f = ezgfx::wgpu::read_spirv(std::io::Cursor::new(&F_SPIRV[..])).unwrap();

                    (v, f)
                };

                let (v, f) =
                {
                    let v = ctx.device.create_shader_module(&v);
                    let f = ctx.device.create_shader_module(&f);

                    (v, f)
                };

                // -- create pipeline --
                let pipeline = ctx.device
                    .create_render_pipeline
                    (
                        &ezgfx::wgpu::RenderPipelineDescriptor
                        {
                            layout: &pip_layout,
                            vertex_stage: ezgfx::wgpu::ProgrammableStageDescriptor
                            {
                                module: &v,
                                entry_point: "main"
                            },
                            fragment_stage: Some(ezgfx::wgpu::ProgrammableStageDescriptor
                            {
                                module: &f,
                                entry_point: "main"
                            }),
                            rasterization_state: Some(ezgfx::wgpu::RasterizationStateDescriptor
                            {
                                front_face: ezgfx::wgpu::FrontFace::Ccw,
                                cull_mode: ezgfx::wgpu::CullMode::Back,
                                depth_bias: 0,
                                depth_bias_slope_scale: 0.0,
                                depth_bias_clamp: 0.0
                            }),
                            color_states: 
                            &[
                                ezgfx::wgpu::ColorStateDescriptor
                                {
                                    format: ezgfx::wgpu::TextureFormat::Bgra8UnormSrgb,
                                    color_blend: ezgfx::wgpu::BlendDescriptor::REPLACE,
                                    alpha_blend: ezgfx::wgpu::BlendDescriptor::REPLACE,
                                    write_mask: ezgfx::wgpu::ColorWrite::ALL
                                }
                            ],
                            primitive_topology: ezgfx::wgpu::PrimitiveTopology::TriangleList,
                            depth_stencil_state: None,
                            vertex_state: ezgfx::wgpu::VertexStateDescriptor
                            {
                                index_format: <#i_type as ezgfx::Index>::FORMAT,
                                vertex_buffers:
                                &[
                                    ezgfx::wgpu::VertexBufferDescriptor
                                    {
                                        stride: <#v_type as ezgfx::BufMember>::SIZE as ezgfx::wgpu::BufferAddress,
                                        step_mode: ezgfx::wgpu::InputStepMode::Vertex,
                                        attributes: <#v_type as ezgfx::Vertex>::DESC
                                    }
                                ]
                            },
                            sample_count: 1,
                            sample_mask: !0,
                            alpha_to_coverage_enabled: false
                        }
                    );

                pipeline
            }
        }
    };

    TokenStream::from(expanded)
}

fn compile(stage: ShaderKind, path: &str) -> CompilationArtifact
{
    let rpath = std::path::Path::new // fallback: std::env::current_dir().unwrap().join(path)
    (
        std::env::var("CARGO_MANIFEST_DIR")
            .unwrap()
            .as_str()
    )
    .join("src")
    .join(path);

    let source = std::fs::read_to_string(&rpath)
        .expect(format!("shader not found: {:?}", rpath).as_str());
    
    Compiler::new()
        .unwrap()
        .compile_into_spirv(source.as_str(), stage, path, "main", None)
        .unwrap()
}

fn uniforms(spirv: &[u8]) -> Vec<Uniform>
{
    use ezgfx_core::spirv_reflect::*;

    match ShaderModule::load_u8_data(spirv)
    {
        Ok(ref mut a) =>
        {
            let b = a.enumerate_descriptor_bindings(None).unwrap();

            b.iter().map
            (
                |c|
                {
                    let name = match &c.type_description
                    {
                        Some(d) =>
                        {
                            if d.type_name.is_empty() { c.name.clone() } else { d.type_name.clone() }
                        },
                        _ => c.name.clone()
                    };
                    let name = Ident::new
                    (
                        format!("{:?}_{}", a.get_shader_stage(), name).to_snake_case().as_str(),
                        Span::call_site()
                    );

                    Uniform { name, set: c.set, binding: c.binding }
                }
            )
            .collect()
        },
        Err(e) => panic!(e)
    }
}

#[derive(Debug)]
struct Uniform
{
    pub name: Ident,
    pub set: u32,
    pub binding: u32,
}

fn bind_group_layout_descriptors(uniforms: &[&Vec<Uniform>]) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<usize>)
{
    let mut tks_layout =                        // token streams for layout
        Vec::<proc_macro2::TokenStream>::new();

    let mut tks_bind =                          // token streams for binding
        Vec::<proc_macro2::TokenStream>::new();

    for uniform_vec in uniforms.iter()
    {
        for uniform in uniform_vec.iter()
        {
            let set = uniform.set as usize;     // set
            let bin = uniform.binding;          // binding
            let sta =                           // shader stage
            if uniform.name.to_string().starts_with("vertex")
            {
                quote! {ezgfx::wgpu::ShaderStage::VERTEX}
            }
            else if uniform.name.to_string().starts_with("fragment")
            {
                quote! {ezgfx::wgpu::ShaderStage::FRAGMENT}
            }
            else
            {
                panic!("expected input shader to be vertex or fragment!")
            };
            let nam = &uniform.name;            // name

            let siz = std::cmp::max(tks_layout.len(), set + 1);
            tks_layout.resize(siz, quote! {});  // resize
            tks_bind.resize(siz, quote! {});

            let pre = &tks_layout[set];         // previous tokens
            tks_layout[set] = quote!
            {
                #pre
                ezgfx::wgpu::BindGroupLayoutEntry
                {
                    binding: #bin,
                    visibility: #sta,
                    ty: #nam.ty()
                },
            };

            let pre = &tks_bind[set];
            tks_bind[set] = quote!
            {
                #pre
                ezgfx::wgpu::Binding
                {
                    binding: #bin,
                    resource: #nam.resource()
                },
            };
        }
    }

    let mut out_tks_layout =                    // out token streams layout
        Vec::<proc_macro2::TokenStream>::new();
    let mut out_tks_bind =                      // out token streams bind
        Vec::<proc_macro2::TokenStream>::new();
    let mut out_ind =                           // out set indices
        Vec::<usize>::new(); 

    for (i, t) in tks_layout.iter().enumerate()        // remove empties
    {
        if t.is_empty()
        {
            continue;
        }
        out_tks_layout.push(t.to_owned());
        out_tks_bind.push(tks_bind[i].to_owned());
        out_ind.push(i);
    }
    (out_tks_layout, out_tks_bind, out_ind)
}