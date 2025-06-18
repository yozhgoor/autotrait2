#![doc = include_str!("../README.md")]

use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn autotrait(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemImpl);

    let (trait_, error) = generate_trait(&item);
    let error = error.map(syn::Error::into_compile_error);

    let res = quote! {
        #trait_
        #item
        #error
    };

    res.into()
}

fn generate_trait(item: &syn::ItemImpl) -> (syn::ItemTrait, Option<syn::Error>) {
    let (items, errors): (Vec<_>, Vec<_>) = item
        .items
        .iter()
        .flat_map(|item| match item {
            syn::ImplItem::Fn(item_fn) => Some(Ok(syn::TraitItem::Fn(syn::TraitItemFn {
                attrs: item_fn.attrs.clone(),
                sig: item_fn.sig.clone(),
                default: None,
                semi_token: Some(Default::default()),
            }))),
            syn::ImplItem::Const(_) | syn::ImplItem::Type(_) => Some(Err(syn::Error::new(
                item.span(),
                "cannot be used with autotrait",
            ))),
            _ => None,
        })
        .partition(|x| x.is_ok());

    let mut it = errors.into_iter().map(|x| x.err().unwrap());
    let mut error = it.next();
    if let Some(err) = error.as_mut() {
        err.extend(it);
    }

    (
        syn::ItemTrait {
            attrs: item.attrs.clone(),
            vis: syn::Visibility::Public(Default::default()),
            unsafety: item.unsafety,
            auto_token: None,
            restriction: None,
            trait_token: Default::default(),
            ident: item
                .trait_
                .as_ref()
                .and_then(|(_, path, _)| path.get_ident())
                .unwrap()
                .clone(),
            generics: item.generics.clone(),
            colon_token: None,
            supertraits: Default::default(),
            brace_token: item.brace_token,
            items: items.into_iter().flat_map(Result::ok).collect(),
        },
        error,
    )
}
