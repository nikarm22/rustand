use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, ItemImpl, Type, parse_macro_input};

/// Generates a `thread_local!` global store for the decorated struct.
/// The struct must implement `Default`.
/// It provides a `::store()` associated function to easily retrieve the store.
#[proc_macro_attribute]
pub fn global_store(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let store_name = format_ident!("__RUSTAND_{}_STORE", name.to_string().to_uppercase());

    let expanded = quote! {
        #input

        ::std::thread_local! {
            static #store_name: ::std::cell::OnceCell<::rustand::Store<#name>> = const { ::std::cell::OnceCell::new() };
        }

        impl #name {
            /// Returns the global thread-local store instance for this type.
            pub fn store() -> ::rustand::Store<#name> {
                #store_name.with(|cell| {
                    cell.get_or_init(|| ::rustand::Store::new(<#name as ::std::default::Default>::default())).clone()
                })
            }
        }
    };
    TokenStream::from(expanded)
}

/// Allows writing an `impl Store<State>` block directly.
/// This macro rewrites it into an Extension Trait to satisfy Rust's orphan rules,
/// allowing seamless `store.method()` syntax.
#[proc_macro_attribute]
#[allow(clippy::collapsible_if)]
pub fn store_actions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let self_ty = &input.self_ty;

    // Attempt to extract the inner type `T` from `Store<T>` to create a nice trait name (e.g., StateStoreExt)
    let mut trait_name = format_ident!("StoreExt");
    if let Type::Path(type_path) = &**self_ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first()
                {
                    if let Some(inner_seg) = inner_path.path.segments.last() {
                        trait_name = format_ident!("{}StoreExt", inner_seg.ident);
                    }
                }
            }
        }
    }

    let mut trait_items = Vec::new();
    let mut impl_items = Vec::new();

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            let sig = &method.sig;
            trait_items.push(quote! { #sig; });
            impl_items.push(quote! { #method });
        }
    }

    let expanded = quote! {
        pub trait #trait_name {
            #(#trait_items)*
        }

        impl #trait_name for #self_ty {
            #(#impl_items)*
        }
    };
    TokenStream::from(expanded)
}
