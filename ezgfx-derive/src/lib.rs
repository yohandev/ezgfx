use proc_macro::*;

mod render_pipeline;
mod pipeline;
mod vertex;
mod util;

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