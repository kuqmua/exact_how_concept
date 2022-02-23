use std::fs;
use std::path::Path;

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
    // println!("attr: \"{}\"", attr.to_string());
    // println!("attr: \"{:#?}\"", attr);

    let question_mark_count = item.to_string().matches("?").count();
    // println!("question_mark_count: \"{}\"", question_mark_count);
    println!("item: \"{}\"", item);
    // println!("item: \"{:#?}\"", item);
    //I CAN EXTEND THE ITEM!
    // let functions_that_can_throw_vec: Vec<Ident> = Vec::new();
    for i in attr {
        // println!("i {}", i);
        // println!("i {}", i.to_string());

        // println!("ilen {}", "./src/i_can_throw_one.rs".len());
        // println!("istrlen {}", i.to_string().len());
        let handle = i.to_string();
        let path = handle[1..handle.len() - 1].to_string(); //remove ""
                                                            // println!("path!{}!", path);
        match fs::read_to_string(&path) {
            Err(e) => panic!("file:  error: {e}"),
            Ok(file) => {
                let token_stream: proc_macro::TokenStream = file
                    .parse()
                    .expect("cannot parse file into proc_macro::TokenStream");
                println!("tokenstream {}", token_stream);
                println!("tokenstream {:#?}", token_stream);
            }
        }
    }

    // quote! {
    //     //kekw
    // }
    // .into()
    //and i can extend not only function! i can write struct+ enum + function!
    item
}
// pub fn wrapper() -> Result<(), WrapperExactHowError> {
//     use crate::i_can_throw_one::i_can_throw_one;
//     // use crate::i_can_throw_one::ICanThrowOneError;
//     use crate::i_can_throw_two::i_can_throw_two;
//     // use crate::i_can_throw_two::ICanThrowTwoError;
//     match i_can_throw_one() {
//         Ok(_) => {}
//         Err(e) => {
//             return Err(WrapperExactHowError::ICanThrowOneError(e));
//         }
//     }
//     match i_can_throw_two() {
//         Ok(_) => {}
//         Err(e) => {
//             return Err(WrapperExactHowError::ICanThrowTwoError(e));
//         }
//     }
//     Ok(())
// }
