use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
#[allow(unused)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}
#[allow(unused)]
#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}
pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()else {
        panic!("AutoDeref only works on structs ");
    };

    println!("宏走了这个");
    // println!("{:?}  {:?}", ident, generics);
    println!("{:?}", mutable);
    // println!("{:?}", field);
    // println!("{:#?}", fields);
    let (fd, ty) = match field {
        Some(v) => {
            let f = fields.iter().find(|f| f.ident.as_ref().unwrap() == &v).unwrap_or_else(|| {
                if fields.len() > 1 {
                    let f = fields.iter().next().unwrap();
                    f
                } else {
                    panic!("AutoDeref only works on structs with 1 field or with field attribute");
                }
            });
            (f.ident.as_ref().unwrap().clone(), &f.ty)
        }
        None => {
            if fields.len() == 1 {
                let f = fields.iter().next().unwrap();
                (f.ident.as_ref().unwrap().clone(), &f.ty)
            } else {
                panic!("field {:?} not found in the data structure", field)
            }
        }
    };
    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics{
            type Target = #ty;
            fn deref(&self)->&Self::Target{
                &self.#fd
            }
        }

    }];
    if mutable {
        code.push(quote! {
         impl #generics std::ops::DerefMut for #ident #generics{
            fn deref_mut(&mut self)->&mut Self::Target{
                &mut self.#fd
            }
        }
     })
    }
    // println!("{:#?}", fd);
    quote! {
        #(#code)*
    }
}