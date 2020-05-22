use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn impl_uniform(_: TokenStream, item: TokenStream) -> TokenStream
{
    // parse item
    let input = parse_macro_input!(item as ItemStruct);

    let vis = input.vis;
    let name = input.ident;
    let (impl_gene, type_gene, where_clause) = input.generics.split_for_impl();
    
    let fields = input.fields.iter();
    let (size, field_names, field_decl) = parse_fields(&input.fields);

    // impl trait
    let expanded = quote!
    {
        #vis struct #name #type_gene #where_clause
        {
            // -- provided by user --
            #(#fields,)*

            // -- generated by macro --
            buf: ezgfx::wgpu::Buffer
        }

        // -- generated by macro --
        impl #impl_gene ezgfx::Uniform for #name #type_gene #where_clause
        {
            fn ty(&self) -> ezgfx::wgpu::BindingType
            {
                ezgfx::wgpu::BindingType::UniformBuffer { dynamic: false }
            }

            fn resource(&self) -> ezgfx::wgpu::BindingResource
            {
                ezgfx::wgpu::BindingResource::Buffer
                {
                    buffer: &self.buf,
                    range: 0..Self::SIZE as ezgfx::wgpu::BufferAddress
                }
            }
        }

        // -- generated by macro --
        impl #impl_gene #name #type_gene #where_clause
        {
            const SIZE: usize = 42; // macro eval

            pub fn create(render: &ezgfx::RenderQueue, #field_decl) -> Self
            {
                #[repr(C)]
                #[derive(Copy, Clone)]
                struct Data // same exact as user-provided def, with name 'Data'
                {
                   #field_decl
                }

                unsafe impl ezgfx::bytemuck::Pod for Data {}
                unsafe impl ezgfx::bytemuck::Zeroable for Data {}
                
                let data = Data { #field_names };

                let buf = render.device.create_buffer_with_data
                (
                    ezgfx::bytemuck::cast_slice(&[data]),
                    ezgfx::wgpu::BufferUsage::UNIFORM | ezgfx::wgpu::BufferUsage::COPY_DST
                );

                Self
                {
                    #field_names
                    buf,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_fields(fields: &Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream)
{
    let named = match fields
    {
        Fields::Named(a) => &a.named,
        _ => panic!("uniform decl must be a struct with named fields!")
    };

    // indexers
    let mut siz = quote! { 0 }; // offset
    let mut nam = quote! { };   // empty out
    let mut dec = quote! { };   // declarations

    // loop through every field in struct
    for field in named
    {
        let ty = &field.ty;    // typename
        let id = field         // name
            .ident
            .as_ref()
            .unwrap();

        nam = quote!            // update output code
        {
            #nam
            #id,
        };
        dec = quote!
        {
            #dec
            #id: #ty,
        };

        let add =               // size for this loop
            quote! { + <#ty as ezgfx::BufMember>::SIZE };
        
        siz =                   // update total size
            quote! { #siz #add };
    }

    (siz, nam, dec)             // return output
}