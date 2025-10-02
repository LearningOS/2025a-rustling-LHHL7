// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


#[macro_export] //注解表明只要导入了定义这个宏的 crate  该宏就应该是可用的。
// 如果没有该注解，这个宏不能被引入作用域。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    //在 Rust 里，所有宏调用（无论是内置宏还是自定义宏）都 以  !  结尾，
    // 这是语法规定，用来告诉编译器“这是宏，不是函数”。
}
