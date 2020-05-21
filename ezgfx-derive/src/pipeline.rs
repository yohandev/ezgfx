use proc_macro::TokenStream;
use syn::*;

pub fn impl_pipeline(args: TokenStream, item: TokenStream) -> TokenStream
{
    let args = parse_macro_input!(args as AttributeArgs);

    match get_type(args).as_str()
    {
        "render" => crate::render_pipeline::process(item),
        "compute" => panic!("compute pipeline not yet implemented!"),
        _ => panic!("expected either 'render' or 'compute'(no quotes)")
    }
}

fn get_type(raw: Vec<NestedMeta>) -> String
{
    assert!(raw.len() == 1, "expected 1 argument! 'render' or 'compute'");

    let err = || panic!("expected either 'render' or 'compute'(no quotes)");

    match &raw[0]
    {
        NestedMeta::Meta(a) =>
        {
            match a
            {
                Meta::Path(b) =>
                {
                    match b.get_ident()
                    {
                        Some(c) => c.to_string(),
                        None => err()
                    }
                }
                _ => err()
            }
        }
        _ => err()
    }
}