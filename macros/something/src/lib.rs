use std::fs;

use convert_case::Case;
use convert_case::Casing;

use quote::quote;

use syn;
use syn::ReturnType;
// use syn::TraitItem;

use proc_macro::TokenStream;

use syn::Data;
use syn::Ident;

//copy of print_type_from_config
#[proc_macro_derive(ExactHowConceptDerive)]
pub fn derive_exact_how_concept(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); //if need to print ast use  instead of syn="1.0.75"
    println!("{:#?}", ast);
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let generated = quote! {};
    generated.into()
    ////////////////////
    // let trait_name: Ident;
    // let function_vec_idents: Vec<(Ident, ReturnType)>;
    // match fs::read_to_string("./src/traits/provider_kind_from_config_trait.rs") {
    //     Err(e) => panic!("file:  error: {e}"),
    //     Ok(file) => {
    //         let token_stream: proc_macro::TokenStream = file
    //             .parse()
    //             .expect("cannot parse file into proc_macro::TokenStream");
    //         let trait_ast: syn::ItemTrait = syn::parse(token_stream)
    //             .expect("cannot parse token_stream from file into syn::ItemTrait");
    //         trait_name = trait_ast.ident;
    //         function_vec_idents = trait_ast
    //             .items
    //             .iter()
    //             .filter_map(|trait_item| match trait_item {
    //                 TraitItem::Method(trait_item_method) => Some((
    //                     trait_item_method.sig.ident.clone(),
    //                     trait_item_method.sig.output.clone(),
    //                 )),
    //                 _ => None,
    //             })
    //             .collect();
    //     }
    // }
    //     //                 quote! {
    //     //                     impl From<#inner_enum_type> for #struct_ident {
    //     //                         fn from(error: #inner_enum_type) -> Self {
    //     //                             #struct_ident {
    //     //                                 source: Box::new(#ident::#variant(
    //     //                                     error,
    //     //                                 )),
    //     //                                 where_was: WhereWas {
    //     //                                     file: file!(),
    //     //                                     line: line!(),
    //     //                                     column: column!(),
    //     //                                 },
    //     //                             }
    //     //                         }
    //     //                     }
    //     //                 }
    // }
    // let generated = quote! {
    //     impl #trait_name for #ident {
    //         #(#function_quote_vec_ident)*
    //     }
    // };
    // generated.into()
}

// #[proc_macro_derive(SomethingDerive)]
// pub fn derive_something(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     println!("{:#?}", ast);
//     quote! {}.into()
// }

// #[derive(SomethingDerive)]
// pub enum Something {
//     One,
//     Two,
// }

// //что должно быть тут?
// pub fn do_something() -> i32 {
//     1 + 2
// }
