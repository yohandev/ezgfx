use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn impl_vertex(item: TokenStream) -> TokenStream
{
    // parse item
    let input = parse_macro_input!(item as ItemStruct);

    // impl trait
    let expanded = quote!
    {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default, ezgfx::Vertex)]
        #input
    };

    TokenStream::from(expanded)
}

pub fn derive_vertex(tokens: TokenStream) -> TokenStream
{
    // parse tokens
    let input = parse_macro_input!(tokens as DeriveInput);

    // dissect input
    let name = input.ident;
    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();

    // logic
    let (attr, size) = get_attribute_descriptors(&input.data);

    // impl trait
    let expanded = quote!
    {
        impl #impl_gene ezgfx::Vertex for #name #type_gene #where_clause
        {
            const DESC: &'static [ezgfx::wgpu::VertexAttributeDescriptor] =
            &[
                #attr
            ];
        }

        impl #impl_gene ezgfx::BufMember for #name #type_gene #where_clause
        {
            const SIZE: usize = #size;
        }

        unsafe impl #impl_gene ezgfx::bytemuck::Pod for #name #type_gene #where_clause{ }
        unsafe impl #impl_gene ezgfx::bytemuck::Zeroable for #name #type_gene #where_clause { }
    };

    TokenStream::from(expanded)
}

fn get_attribute_descriptors(data: &Data) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
{
    // get all the struct's fields
    let fields = match data
    {
        Data::Struct
        (
            DataStruct
            {
                fields: Fields::Named(fields),
                ..
            }
        ) => &fields.named,
        Data::Struct
        (
            DataStruct
            {
                fields: Fields::Unnamed(fields),
                ..
            }
        ) => &fields.unnamed,
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
            quote! { + <#typ as ezgfx::BufMember>::SIZE };
        
        off =                   // update buffer offset
            quote! { #off #add };

        loc += 1;               // increment shader location
    }

    (out, off)                  // return output
}