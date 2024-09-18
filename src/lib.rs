mod enum_from;
mod process_enum_darling;

use proc_macro::TokenStream;
use enum_from::process_enum_from;
use process_enum_darling::process_enum_darling;

/// 这段代码定义了一个名为 EnumFrom 的过程宏，它用于为 Rust 中的枚举类型自动生成 From 实现。过程宏是一种在编译时运行的宏，它接收 Rust 代码作为输入，然后生成新的 Rust 代码作为输出。
// 首先，宏通过 #[proc_macro_derive(EnumFrom)] 属性声明为过程宏，并定义了一个名为 derive_enum_from 的函数，该函数接收一个 TokenStream 类型的参数，这是 Rust 编译器内部表示代码的一种方式。
// 在函数内部，使用 syn::parse_macro_input! 宏将输入的 TokenStream 解析为一个 syn::DeriveInput 结构体，这个结构体包含了关于输入代码的信息，比如标识符、数据等。
// 接下来，代码获取了枚举类型的标识符，并检查输入的数据是否为枚举类型。如果不是枚举类型，宏会触发一个 panic，提示 EnumFrom 只能在枚举上使用。
// 如果输入是枚举类型，代码会获取枚举的所有变体，并遍历这些变体。对于每个变体，它会检查变体的字段是否未命名且数量为 1。如果条件满足，它会生成一个 From 实现，允许从这个字段的类型转换到枚举类型。
// 最后，所有生成的 From 实现会被合并，并转换回 TokenStream 类型，然后返回给编译器。这样，当用户在他们的代码中使用 #[derive(EnumFrom)] 属性时，编译器会调用这个过程宏，并将生成的 From 实现插入到用户的代码中。
// 这个宏的一个关键点是它只处理未命名字段的变体，并且这些字段的数量必须为 1。这是因为 From 实现需要一个单一的值来构造枚举变体。如果变体有命名字段或多个未命名字段，宏将不会生成 From 实现。
// 这个宏的使用可以大大减少为枚举类型手动编写 From 实现的重复工作，特别是在处理大量变体时，可以提高代码的可维护性和开发效率。
// proc  macro crate
// for  enum .we'd like to generate From impls for each variant.
#[proc_macro_derive(EnumFrom)] // 定义一个过程宏 EnumFrom
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput); // 解析输入的TokenStream为DeriveInput结构体
    println!("{:#?}",input);
    // .into() // 转换为TokenStream并返回
    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input:TokenStream)->TokenStream{
    // 解析输入的TokenStream为DeriveInput结构体
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_darling(input).into()
}


