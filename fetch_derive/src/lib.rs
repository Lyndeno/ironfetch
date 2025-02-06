use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(fetch), supports(struct_named, struct_newtype))]
struct DeriveMacroArgs {
    #[darling(map = Some)]
    name: Option<String>,
}

#[proc_macro_derive(Fetch, attributes(fetch))]
pub fn fetch_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_fetch_macro(&ast)
}

fn impl_fetch_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_ident = &ast.ident;
    let struct_ident_str = struct_ident.clone().to_string();

    let args = match DeriveMacroArgs::from_derive_input(&ast) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let DeriveMacroArgs { name } = args;

    let name_string = if let Some(n) = name {
        n.clone()
    } else {
        struct_ident_str
    };

    let new_str = name_string.as_str();

    let gen = quote! {
        impl Fetch for #struct_ident {
            fn name(&self) -> &'static str {
                #new_str
            }
        }
    };
    gen.into()
}
