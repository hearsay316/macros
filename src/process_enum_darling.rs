use darling::{FromDeriveInput, ast::{Data, Fields}, FromField, FromVariant};
use darling::ast::Style;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput };



#[derive(Debug,FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
}


#[derive(Debug,FromVariant )]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantFields>,
}
#[derive(Debug,FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data:Data::Enum(data)
    } = EnumFromDarling::from_derive_input(&input).unwrap()else {
        panic!("EnumFromDarling only works on enums");
    };
    println!("{:#?}",ident);
    println!("{:#?}",generics);
    println!("这个是测试");
    println!("{:#?}",data);

    let from_impls = data.iter().map(|variant|{
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len()==1=>{
                let field = variant.fields.fields.first().expect("should have 1 field");
                let ty = &field.ty;
                quote!{
                    impl #generics From<#ty> for #ident #generics{
                        fn from(v:#ty)->Self{
                            #ident::#var(v)
                        }
                    }
                }
            },
            _ =>quote! {}
        }
    });
    quote! {
        #(#from_impls)*
    }
}