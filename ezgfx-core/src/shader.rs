use glsl::syntax::*;
use glsl::parser::*;
use wgpu::*;

/// represents a bindable shader resource
pub(crate) trait ShaderResource
{
    const DESC: BindGroupLayoutDescriptor<'static>;
    fn desc<'a>() -> &'a BindGroupLayoutDescriptor<'a>;

    fn bind_group() -> BindGroup;
    fn layout() -> BindGroupLayout;
}

pub(crate) trait ShaderModule
{
    const SOURCE: str;

    fn compile();
    fn resource(i: usize);
}

fn get_resources(src: &str)
{
    let stage = TranslationUnit::parse(src);

    if stage.is_err()
    {
        panic!("shader err: {}", stage.unwrap_err());
    }
    let stage = stage.unwrap();

    for stat in stage
    {
        match stat
        {
            ExternalDeclaration::Preprocessor(_) => {}
            ExternalDeclaration::FunctionDefinition(_) => {}
            ExternalDeclaration::Declaration(_) => {}
        }
    }
}