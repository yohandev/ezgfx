use wgpu::*;

// /// represents a bindable shader resource
// pub(crate) trait ShaderResource
// {
//     const DESC: BindGroupLayoutDescriptor<'static>;
//     fn desc<'a>() -> &'a BindGroupLayoutDescriptor<'a>;

//     fn bind_group() -> BindGroup;
//     fn layout() -> BindGroupLayout;
// }

// pub(crate) trait ShaderModule
// {
//     const SOURCE: str;

//     fn compile();
//     fn resource(i: usize);
// }

pub fn list_layout(path:&str, src: &str)
{
    use shaderc::*;
    use spirv_reflect::*;

    let mut compiler = Compiler::new().unwrap();
    let compiled = compiler.compile_into_spirv(src, ShaderKind::Vertex, path, "main", None).unwrap();
    
    match ShaderModule::load_u32_data(compiled.as_binary())
    {
        Ok(ref mut a) =>
        {
            let b = a.enumerate_descriptor_bindings(None).unwrap();

            for c in &b
            {
                let name = match &c.type_description
                {
                    Some(d) =>
                    {
                        if d.type_name.is_empty()
                        {
                            &c.name
                        }
                        else
                        {
                            &d.type_name
                        }
                    },
                    _ => &c.name
                };
                println!("{:?} {} is (set = {}, binding = {})", c.resource_type, name, c.set, c.binding);
            }
        },
        Err(e) => panic!(e)
    };
}