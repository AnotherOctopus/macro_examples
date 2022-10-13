#![allow(dead_code)]
#![allow(unused_variables)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Getters)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let mut fields = Vec::new();
    let mut fieldnames = Vec::new();
    match input.data {
        syn::Data::Struct(data_struct) => {
            match data_struct.fields{
                syn::Fields::Named(named_field) => {
                    for field in named_field.named{
                        let fieldname = field.ident.unwrap().clone();
                        fields.push(fieldname.clone());
                        fieldnames.push(format_ident!("get_{}", fieldname));
                    }
                },
                _ => panic!("Only Handling Names fields")
            }
        },
        _ => panic!("Only Handling Structs")

    }
    let structname = input.ident;
    let generated: TokenStream = quote!(
        impl #structname {
            #(
                fn #fieldnames(self) -> u32{
                    self.#fields
                }
            )*
        }
    ).into();
    println!("{}", generated);
    generated
}