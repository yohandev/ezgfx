use proc_macro::TokenStream;
use spirv_reflect::types::*;
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