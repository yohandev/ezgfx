use syn::*;

pub fn get_type_value<'a>(look_for: &str, items: &'a Vec<ImplItem>) -> &'a proc_macro2::Ident
{
    for item in items
    {
        match item
        {
            ImplItem::Type(a) =>
            {
                if look_for == a.ident.to_string()
                {
                    if let Type::Path(b) = &a.ty
                    {
                        return &b.path.segments[0].ident;
                    }
                    panic!("expected a type at token: {}", look_for);
                }
            }
            _ => {}
        }
    }
    panic!("missing trait type '{}'", look_for);
}

pub fn get_const_value<'a>(look_for: &str, ty: &str, items: &'a Vec<ImplItem>) -> &'a Lit
{
    for item in items
    {
        match item
        {
            ImplItem::Const(a) =>
            {
                if look_for == a.ident.to_string()
                {
                    if let Type::Path(b) = &a.ty
                    {
                        let t = &b.path.segments[0].ident;
                        if t != ty
                        {
                            panic!("mismatched type! expected {} but found {}", ty, t);
                        }
                    }
                    if let Expr::Lit(b) = &a.expr
                    {
                        return &b.lit;
                    }
                    panic!("expected a constant of type {} at token: {}", ty, look_for);
                }
            }
            _ => {}
        }
    }
    panic!("missing trait type '{}'", look_for);
}