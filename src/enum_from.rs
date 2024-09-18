use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    let ident = input.ident; // 获取枚举的标识符
    let generics = input.generics;
    let variants = match input.data { // 匹配输入数据为枚举类型
        syn::Data::Enum(data) => data.variants, // 获取枚举的所有变体
        _ => panic!("EnumFrom only works on enums "), // 如果不是枚举类型则报错
    };
    let from_impls = variants.iter().map(|variant| { // 遍历每个变体并生成From实现
        let var = &variant.ident; // 获取变体的标识符
        match &variant.fields { // 匹配变体的字段
            Fields::Unnamed(fields) => { // 如果字段未命名
                if fields.unnamed.len() != 1 { // 如果未命名字段的数量不为1，则不生成实现
                    quote! {}
                } else { // 如果只有一个未命名字段
                    let field = fields.unnamed.first().expect("应该只有一个"); // 获取第一个字段
                    let ty = &field.ty; // 获取字段类型
                    quote! { // 生成From实现
                        impl #generics From<#ty> for #ident #generics{
                            fn from(v:#ty)-> Self{
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            _ => { // 如果字段已命名或其他情况，则不生成实现
                quote! {}
            }
        }
    });
    quote! { // 将所有生成的From实现合并
        #(#from_impls)*
    }
}


// pub fn derive_enum_from1(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as syn::DeriveInput); // 解析输入的TokenStream为DeriveInput结构体
//     println!("{:#?}",input);
//     let ident = input.ident; // 获取枚举的标识符
//     let generics = input.generics.params.first();
//     let generics_ident =generics.map(|da|{
//         match da {
//             GenericParam::Type(data)=> &data.ident,
//             _=>panic!("没有取出来 generics_ident的 ident")
//         }
//
//     }).unwrap();
//
//     let variants = match input.data{ // 匹配输入数据为枚举类型
//         syn::Data::Enum(data)=>data.variants, // 获取枚举的所有变体
//         _=>panic!("EnumFrom only works on enums "), // 如果不是枚举类型则报错
//     };
//     let from_impls = variants.iter().map( // 遍历每个变体并生成From实现
//                                           |variant|{
//                                               let var = &variant.ident; // 获取变体的标识符
//                                               match &variant.fields { // 匹配变体的字段
//                                                   Fields::Unnamed(fields)=>{ // 如果字段未命名
//                                                       if fields.unnamed.len()!=1{ // 如果未命名字段的数量不为1，则不生成实现
//                                                           quote!{}
//                                                       }else { // 如果只有一个未命名字段
//                                                           let field = fields.unnamed.first().expect("应该只有一个"); // 获取第一个字段
//                                                           let ty = &field.ty; // 获取字段类型
//                                                           quote! { // 生成From实现
//                             impl<#generics_ident> From<#ty> for #ident<#generics_ident>{
//                                 fn from(v:#ty)-> Self{
//                                     #ident::#var(v)
//                                 }
//                             }
//                         }
//                                                       }
//                                                   }
//                                                   _=>{ // 如果字段已命名或其他情况，则不生成实现
//                                                       quote!{}
//                                                   }
//                                               }
//                                           });
//     (quote!{ // 将所有生成的From实现合并
//         #(#from_impls)*
//     })
//         .into() // 转换为TokenStream并返回
// }