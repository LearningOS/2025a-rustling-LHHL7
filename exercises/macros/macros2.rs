// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.




//宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，
// 或将其引入作用域，而函数则可以在任何地方定义和调用。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}
