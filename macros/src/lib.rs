use proc_macro::*;

mod vertex;

#[proc_macro_attribute]
pub fn vertex(_: TokenStream, item: TokenStream) -> TokenStream
{
    vertex::impl_vertex(item)
}