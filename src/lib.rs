extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::*;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IterStruct)]
pub fn derive_macro_iterate_over_struct(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as _);
    TokenStream::from(match impl_iterator(ast) {
        Ok(it) => it,
        Err(err) => panic!(err),
    })
}

fn impl_iterator(ast: DeriveInput) -> Result<TokenStream> {
    Ok({
        let name = ast.ident;
        let fields = match ast.data {
            Data::Enum(DataEnum {
                enum_token: token::Enum { span },
                ..
            })
            | Data::Union(DataUnion {
                union_token: token::Union { span },
                ..
            }) => {
                return Err(Error::new(span, "expected a struct type"));
            }

            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => it,
            Data::Struct(_) => {
                panic!("expected a struct with named fields");
            }
        };

        let data_expanded_members = fields.named.into_iter().map(|field| {
            let field_name = field.ident.expect("unreachable");
            let span = field_name.span();
            let field_name_stringified = LitStr::new(&field_name.to_string(), span);
            quote_spanned! { span=>
                (#field_name_stringified, &self.#field_name)
            }
        });

        let expand = quote! {
            impl #name {
                fn iter_struct (self: &'_ Self) -> Vec<(String,String)> {
                    vec![#((#data_expanded_members.0.to_string(), #data_expanded_members.1.to_string()),)*]
                }
            }
        };
        expand.into()
    })
}
