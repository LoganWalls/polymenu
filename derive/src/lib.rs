extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

#[proc_macro_derive(UpdateFromOther)]
pub fn update_from_other_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;
    let field_idents: Vec<Ident> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        _ => panic!("UpdateFromOther can only be derived for structs"),
    };
    let assignments = field_idents.into_iter().map(|i| {
        quote! {
            if other.#i != default.#i {
                self.#i = other.#i;
            }
        }
    });
    quote! {
        impl UpdateFromOther for #ident {
            fn update_from_other(&mut self, other: Self){
                let default = Self::default();
                #(#assignments)*
            }
        }
    }
    .into()
}
