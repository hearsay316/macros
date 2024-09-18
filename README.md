枚举类型 From 实现自动生成器<br>
项目简介
本项目旨在通过一个 Rust 过程宏来自动生成枚举类型的 From 实现，以减少重复代码并提高开发效率。过程宏可以在编译时运行，接收 Rust 代码作为输入，并生成新的 Rust 代码作为输出。
功能特性
自动为枚举变体生成 From 实现：支持未命名字段和命名字段的枚举变体。
减少手动编写 From 实现的工作量：特别是在处理大量变体时，可以显著提高代码的可维护性。
灵活的错误处理：可以根据需要添加更多的错误检查和处理逻辑。
三、使用方法
在你的 Rust 项目中添加本过程宏作为依赖。
在你的枚举类型上使用 #[derive(EnumFrom)] 属性。
编译项目，过程宏会自动生成相应的 From 实现。
四、示例代码
#[derive(EnumFrom)]
enum MyEnum {
Variant1(usize),
Variant2 { field: String },
}

fn main() {
let num: usize = 42;
let enum_variant: MyEnum = num.into();
println!("{:?}", enum_variant); // 输出: Variant1(42)

    let text = String::from("hello");
    let enum_variant: MyEnum = text.into();
    println!("{:?}", enum_variant); // 输出: Variant2 { field: "hello" }
}


五、注意事项
本过程宏目前仅支持每个变体有一个命名字段或一个未命名字段的情况。
对于具有多个命名字段或多个未命名字段的变体，需要根据具体需求调整宏的逻辑。
在实际应用中，可能需要添加更多的错误检查和处理逻辑，以确保转换的安全性。
六、许可证
本项目遵循 MIT 许可证。