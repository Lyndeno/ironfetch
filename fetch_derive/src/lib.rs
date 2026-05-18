use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

/// `#[fetch(name = "...", priority = N, colour = "field")]`
///
/// `name`     — display label; defaults to struct name.
/// `priority` — when present, registers the module with inventory so Machine
///              picks it up automatically. Structs without priority (e.g. helper
///              types that derive Fetch for other reasons) are left unregistered.
/// `colour`   — JSON field name to read the terminal colour from (OsInfo only).
#[derive(FromDeriveInput, Clone)]
#[darling(attributes(fetch), supports(struct_named, struct_newtype))]
struct DeriveMacroArgs {
    #[darling(map = Some)]
    name: Option<String>,
    priority: Option<u32>,
    colour: Option<String>,
}

#[proc_macro_derive(Fetch, attributes(fetch))]
pub fn fetch_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_fetch_macro(&ast)
}

fn impl_fetch_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_ident = &ast.ident;
    let struct_ident_str = struct_ident.to_string();

    let args = match DeriveMacroArgs::from_derive_input(ast) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let DeriveMacroArgs {
        name,
        priority,
        colour,
    } = args;

    let name_string = name.as_deref().unwrap_or(&struct_ident_str).to_string();
    let name_str = name_string.as_str();

    let fetch_impl = quote! {
        impl crate::fetch::Fetch for #struct_ident {
            fn name(&self) -> &'static str {
                #name_str
            }
        }
    };

    // No priority means this struct is a helper type, not a top-level module.
    // Return only the Fetch impl and skip inventory registration.
    let Some(priority) = priority else {
        return fetch_impl.into();
    };

    let colour_override = if let Some(field) = colour {
        quote! {
            fn colour_dyn(val: &::serde_json::Value) -> ::core::option::Option<::std::string::String> {
                val.get(#field)?.as_str().map(::std::string::String::from)
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #fetch_impl

        impl crate::fetch::DynModule for #struct_ident {
            fn load_module() -> ::core::option::Option<Self> {
                Self::new().ok().flatten()
            }
            #colour_override
        }

        ::inventory::submit! {
            crate::fetch::ModuleRegistration {
                key: #name_str,
                priority: #priority,
                load: <#struct_ident as crate::fetch::DynModule>::load_dyn,
                display: <#struct_ident as crate::fetch::DynModule>::display_dyn,
                colour: <#struct_ident as crate::fetch::DynModule>::colour_dyn,
            }
        }
    }
    .into()
}
