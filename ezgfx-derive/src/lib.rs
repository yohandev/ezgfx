use proc_macro::*;

mod pipeline;
mod vertex;

#[proc_macro_attribute]
pub fn vertex(_: TokenStream, item: TokenStream) -> TokenStream
{
    vertex::impl_vertex(item)
}

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(tokens: TokenStream) -> TokenStream
{
    vertex::derive_vertex(tokens)
}

#[proc_macro_attribute]
pub fn pipeline(args: TokenStream, item: TokenStream) -> TokenStream
{
    pipeline::impl_pipeline(args, item)
}

#[proc_macro_attribute]
pub fn pipeline_attr(args: TokenStream, item: TokenStream) -> TokenStream
{
    item
}

// #[proc_macro_derive(Pipeline)]
// pub fn derive_pipeline(tokens: TokenStream) -> TokenStream
// {
//     pipeline::derive_pipeline(tokens)
// }