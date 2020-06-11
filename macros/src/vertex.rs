use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn impl_vertex(item: TokenStream) -> TokenStream
{
    // store struct as-is
    let raw_input = parse_macro_input!(item as ItemStruct);

    // parse item
    let input = raw_input.clone();

    // dissect input
    let name = input.ident;
    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();

    // logic
    let (attr, _) = get_attribute_descriptors(&input.fields);

    // impl trait
    let expanded = quote!
    {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default)]
        #raw_input

        impl #impl_gene ezgfx::Vertex for #name #type_gene #where_clause
        {
            const DESC: &'static [ezgfx::wgpu::VertexAttributeDescriptor] =
            &[
                #attr
            ];
        }

        unsafe impl #impl_gene ezgfx::bytemuck::Pod for #name #type_gene #where_clause{ }
        unsafe impl #impl_gene ezgfx::bytemuck::Zeroable for #name #type_gene #where_clause { }
    };

    TokenStream::from(expanded)
}

fn get_attribute_descriptors(data: &Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
{
    // get all the struct's fields
    let fields = match data
    {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => panic!("expected a struct with fields!"),
    };

    // indexers
    let mut off = quote! { 0 }; // offset
    let mut loc = 0u32;         // location
    let mut out = quote! { };   // empty out

    // loop through every field in struct
    for field in fields
    {
        let typ = &field.ty;    // typename
        
        let new = quote!        // new vertex attribute for this loop
        {
            ezgfx::wgpu::VertexAttributeDescriptor
            {
                offset: (#off) as u64,
                format: <#typ as ezgfx::VertexAttribute>::FORMAT,
                shader_location: #loc
            },
        };
        out = quote!            // update output code
        {
            #out
            #new
        };

        let add =               // vertex attribtue size for this loop
            quote! { + std::mem::size_of::<#typ>() };
        
        off =                   // update buffer offset
            quote! { #off #add };

        loc += 1;               // increment shader location
    }

    (out, off)                  // return output
}