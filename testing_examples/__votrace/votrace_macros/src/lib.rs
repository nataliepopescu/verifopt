use proc_macro::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut imp = parse_macro_input!(item as ItemImpl);

    let ty = &imp.self_ty;
    let tr = &imp
        .trait_
        .as_ref()
        .unwrap()
        .0
        .segments
        .last()
        .unwrap()
        .ident;

    let label = format!("<{} as {}>", quote!(#ty), tr);

    for item in &mut imp.items {
        if let ImplItem::Fn(f) = item {
            let label = format!("{}::{}", label, f.sig.ident);
            let stmts = &f.block.stmts;

            f.block = parse_quote!({
                ::votrace::hit(#label);
                #(#stmts)*
            });
        }
    }

    quote!(#imp).into()
}
