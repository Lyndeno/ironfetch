use darling::{ast::NestedMeta, Error, FromDeriveInput, FromMeta};
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

/// `#[register_module(name = "...", priority = N, colour = "field")]`
///
/// Generates `DynModule` + `inventory::submit!` for structs that implement
/// `Fetch` manually (e.g. because they need a custom `as_fetchlines`).
/// Does not touch the `Fetch` impl — that stays entirely hand-written.
///
/// `name` defaults to the struct name if omitted.
#[derive(FromMeta)]
struct RegisterModuleArgs {
    name: Option<String>,
    priority: u32,
    colour: Option<String>,
}

#[proc_macro_derive(Fetch, attributes(fetch))]
pub fn fetch_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_fetch_macro(&ast)
}

/// Attribute macro for structs that implement `Fetch` manually.
/// Emits only the registration boilerplate; leaves `impl Fetch` untouched.
#[proc_macro_attribute]
pub fn register_module(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };

    let reg_args = match RegisterModuleArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let item: syn::ItemStruct = syn::parse(input).unwrap();
    let struct_ident = &item.ident;
    let struct_ident_str = struct_ident.to_string();

    let name_string = reg_args
        .name
        .as_deref()
        .unwrap_or(&struct_ident_str)
        .to_string();
    let name_str = name_string.as_str();
    let priority = reg_args.priority;

    let colour_override = build_colour_override(reg_args.colour);

    quote! {
        #item

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

    let colour_override = build_colour_override(colour);

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

fn build_colour_override(field: Option<String>) -> proc_macro2::TokenStream {
    match field {
        Some(f) => quote! {
            fn colour_dyn(val: &::serde_json::Value) -> ::core::option::Option<::std::string::String> {
                val.get(#f)?.as_str().map(::std::string::String::from)
            }
        },
        None => quote! {},
    }
}
