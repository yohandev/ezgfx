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

    let v_type = format!("{}", get_type_value("Vertex", &input.items));
    let i_type = format!("{}", get_type_value("Index", &input.items));

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

    let v = vert.as_binary();
    let f = frag.as_binary();

    let vu = uniforms(&v);
    let fu = uniforms(&f);

    let (binding_group_layouts, sets) = bind_group_layout_descriptors(&[&vu, &fu]);
    let binding_group_layout_sets = sets.iter().map(|s| Ident::new(format!("binding_group_layout_set_{}", s).as_str(), Span::call_site()));

    //panic!("{:?}", binding_group_layouts);

    let uniform_names = vu.iter().chain(fu.iter()).map(|u| &u.name);

    let pipeline_name = input.self_ty;
    let pipeline_name_str = quote! {#pipeline_name}.to_string().to_snake_case();

    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();

    //panic!("{:?}", binding_group_layouts);

    let expanded = quote!
    {
        impl #impl_gene #pipeline_name #type_gene #where_clause
        {
            pub fn create(render: &ezgfx::RenderQueue, #(#uniform_names : &ezgfx::Uniform,)*)
            {
                // -- create layout --
                // let bind_layout = render.device.create_bind_group_layout   // bind group layout
                // (
                //     &ezgfx::wgpu::BindGroupLayoutDescriptor
                //     {
                //         bindings:
                //         &[
                //             #(),*
                //         ],
                //         label: Some(format!("{}_bind_group_layout", #name_str).as_str())
                //     }
                // );
                #(
                    let #binding_group_layout_sets = render.device.create_bind_group_layout
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

fn uniforms(spirv: &[u32]) -> Vec<Uniform>
{
    use ezgfx_core::spirv_reflect::*;

    match ShaderModule::load_u32_data(spirv)
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

fn bind_group_layout_descriptors(uniforms: &[&Vec<Uniform>]) -> (Vec<proc_macro2::TokenStream>, Vec<usize>)
{
    let mut tks =                               // token streams
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
            let nam = &uniform.name;             // name

            let siz = std::cmp::max(tks.len(), set + 1);
            tks.resize(siz, quote! {});         // resize
            
            let pre = &tks[set];                // previous tokens
            tks[set] = quote!
            {
                #pre
                ezgfx::wgpu::BindGroupLayoutEntry
                {
                    binding: #bin,
                    visibility: #sta,
                    ty: #nam.ty()
                },
            }
        }
    }

    let mut out_tks =                           // out token streams
        Vec::<proc_macro2::TokenStream>::new();
    let mut out_ind =                           // out set indices
        Vec::<usize>::new(); 

    for (i, t) in tks.iter().enumerate()        // remove empties
    {
        if t.is_empty()
        {
            continue;
        }
        out_tks.push(t.to_owned());
        out_ind.push(i);
    }
    (out_tks, out_ind)
}