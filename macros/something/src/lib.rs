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
        //         where_was: WhereWas {
        //     file: file!(),
        //     line: line!(),
        //     column: column!(),
        // },
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
