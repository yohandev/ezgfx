use proc_macro::TokenStream;
use spirv_reflect::types::*;
use inflector::Inflector;
use proc_macro2::Span;
use spirv_reflect::*;
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
    let frag = compile(ShaderKind::Vertex, f_path.as_str());

    let v = vert.as_binary();
    let f = frag.as_binary();

    let vu = uniforms(&v);
    let fu = uniforms(&f);

    let vu_str = vu.iter().map(|u| format!("{:?}", u));
    let fu_str = fu.iter().map(|u| format!("{:?}", u));

    let name = input.self_ty;
    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();

    let v_ty = vu.iter().map(|v| match v.ty
    {
        ReflectResourceType::Sampler => Ident::new("ezgfx::Sampler", Span::call_site()),
        ReflectResourceType::CombinedImageSampler => Ident::new("not_yet_supported", Span::call_site()),
        ReflectResourceType::ConstantBufferView => Ident::new(v.name.as_str(), Span::call_site())        ,
        ReflectResourceType::ShaderResourceView => Ident::new("ezgfx::Texture", Span::call_site()),
        ReflectResourceType::UnorderedAccessView => Ident::new("not_yet_supported", Span::call_site()),
        ReflectResourceType::Undefined => panic!("undefined shader resource type {}", v.name)
    });
    let v_ty: Vec<proc_macro2::Ident> = v_ty.collect();
    let v_name = vu.iter().map(|v| Ident::new(&v.name.to_snake_case(), Span::call_site()));
    let v_bindings = vu.iter().map(|v| v.binding);

    let expanded = quote!
    {
        impl #impl_gene #name #type_gene #where_clause
        {
            pub fn print_pipeline_info()
            {
                println!("vertex type: {}", #v_type);
                println!("index type: {}", #i_type);

                println!("vertex uniforms:");
                #(println!("{}", #vu_str );)*

                println!("\nfragment uniforms:");
                #(println!("{}", #fu_str );)*
            }

            pub fn create(render: &ezgfx::RenderQueue, #(#v_name : &#v_ty,)*)
            {
                // -- create layout --
                let bind_layout = render.device.create_bind_group_layout   // bind group layout
                (
                    &ezgfx::wgpu::BindGroupLayoutDescriptor
                    {
                        bindings:
                        &[
                            #(
                            ezgfx::wgpu::BindGroupLayoutEntry
                            {
                                binding: #v_bindings,
                                visibility: ezgfx::wgpu::ShaderStage::VERTEX,
                                ty: ezgfx::wgpu::BindingType::UniformBuffer { dynamic: false }
                            }),*
                        ],
                        label: Some("texture_bind_group_layout")
                    }
                );
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
                    Uniform { ty: c.resource_type, name, set: c.set, binding: c.binding }
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
    pub ty: ReflectResourceType,
    pub name: String,
    pub set: u32,
    pub binding: u32
}