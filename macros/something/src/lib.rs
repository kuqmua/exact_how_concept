use std::fs;

use convert_case::Case;
use convert_case::Casing;

use quote::quote;

use syn;
use syn::ReturnType;
// use syn::TraitItem;

use proc_macro::TokenStream;

use syn::token::Question;
use syn::Data;
use syn::Ident;
use syn::Type;

//copy of print_type_from_config
#[proc_macro_derive(ExactHowConceptDerive)]
pub fn derive_exact_how_concept(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_impl_from_for_upper_struct syn::parse(input) failed");
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants,
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let ident = &ast.ident;
    let generated = variants.into_iter().map(|v| {
        let variant = v.ident;
        let inner_enum_type: Type;
        match &v.fields {
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() != 1 {
                    panic!(
                        "fields_unnamed.unnamed != 1, length is {}",
                        fields_unnamed.unnamed.len()
                    );
                }
                inner_enum_type = fields_unnamed.unnamed[0].ty.clone();
            }
            _ => panic!("v.fields is not syn::Fields::Unnamed"),
        }
        quote! {
            impl From<#inner_enum_type> for #ident {
                fn from(error: #inner_enum_type) -> Self {
                    #ident::#variant(error)
                }
            }
        }
    });
    let gen = quote! {
        #(#generated)*
    };
    gen.into()
}

/////
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    let question_mark_count = item.to_string().matches("?").count();
    println!("question_mark_count: \"{}\"", question_mark_count);
    println!("item: \"{}\"", item);
    println!("item: \"{:#?}\"", item);
    //I CAN EXTEND THE ITEM!
    // quote! {
    //     //kekw
    // }
    // .into()
    item
}
