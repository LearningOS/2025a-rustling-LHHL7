// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon(分号) to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
//Rust 默认会对符号（函数、静态变量等）进行名字改编（mangling），
// 也就是把原本简单的函数名加上一串包含模块、泛型、生命周期等信息的“乱码”，生成全局唯一的符号名。
//这样做是为了支持重载、泛型、嵌套模块等特性，但导致链接时找不到按 C 规则导出的简洁符号。因此：
//想让 Rust 函数被 C 代码调用，必须加  #[no_mangle]  来禁用这种改编，保持原名
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias(别名) for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.
//Rust 的“属性”语法：
//在项（函数、结构体、模块等）前面加  #[...]  或  #![...]  的元数据，用来告诉编译器“这一项该怎么处理”。
//例如  #[no_mangle] 、 #[inline] 、 #[derive(Debug)]  都是属性；



extern "Rust" {
    fn my_demo_function(a: u32) -> u32;//extern块只进行了声明 是想要调用外部的
    #[link_name = "my_demo_function"] 
    //link_name  的用法：
//在  extern  块里声明外部函数时，告诉编译器“找符号的时候用这个名字，而不是函数名本身”。
    fn my_demo_function_alias(a: u32) -> u32;
}


mod Foo {
    // No `extern` equals `extern "Rust"`.
    //不写extern时相当于 extern Rust
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {//写了函数体是方便外部调用
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
