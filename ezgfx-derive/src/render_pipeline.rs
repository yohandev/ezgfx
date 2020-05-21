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

    let vert = compile(ShaderKind::Vertex, v_path.as_str());
    let frag = compile(ShaderKind::Vertex, f_path.as_str());

    let v = vert.as_binary();
    let f = vert.as_binary();

    let vu = uniforms(&v);
    let fu = uniforms(&f);

    let name = input.self_ty;

    let expanded = quote!
    {
        impl #name
        {

        }
    };

    TokenStream::from(expanded)
}

fn compile(stage: ShaderKind, path: &str) -> CompilationArtifact
{
    let rpath = std::env::current_dir().unwrap().join(path);

    let source = std::fs::read_to_string(&rpath)
        .expect(format!("shader not found: {:?}", rpath).as_str());
    
    Compiler::new()
        .unwrap()
        .compile_into_spirv(source.as_str(), stage, path, "main", None)
        .unwrap()
}

fn uniforms(spirv: &[u32]) -> Vec<(ReflectResourceType, String, u32, u32)>
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
                    (c.resource_type, name, c.set, c.binding)
                }
            )
            .collect()
        },
        Err(e) => panic!(e)
    }
}