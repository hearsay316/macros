use proc_macro::TokenStream;
use quote::quote;
use syn::Fields;

// proc  macro crate
// for  enum .we'd like to generate From impls for each variant.
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
 // let input = syn::parse_macro_input!(input as syn::DeriveInput);
 //    println!("{:#?}",input);
    // input
    // 取 ident
    let ident = input.ident;
    let variants = match input.data{
        syn::Data::Enum(data)=>data.variants,
        _=>panic!("EnumFrom only works on enums "),
    };
    let from_impls = variants.iter().map(
        |variant|{
            let var = &variant.ident;
            match &variant.fields {
                Fields::Unnamed(fields)=>{
                    if fields.unnamed.len()!=1{
                        quote!{}
                    }else {
                        let field = fields.unnamed.first().expect("应该只有一个");
                        let ty = &field.ty;
                        quote! {
                            impl From<#ty> for #ident{
                                fn from(v:#ty)-> Self{
                                    #ident::#var(v)
                                }
                            }
                        }
                    }
                }
                _=>{
                    quote!{}
                }
            }
        });
    (quote!{
        #(#from_impls)*
    })
    .into()

}
