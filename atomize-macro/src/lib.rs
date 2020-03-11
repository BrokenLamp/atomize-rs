extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use quote::quote;
use syn;

#[proc_macro_hack]
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
