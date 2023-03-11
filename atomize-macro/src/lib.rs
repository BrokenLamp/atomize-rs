extern crate proc_macro;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn a(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::Ident).to_string();

    let hash: u64 = {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    };

    TokenStream::from(quote! {
        Atom::from(#hash)
    })
}
