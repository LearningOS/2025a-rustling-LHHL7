// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


#[macro_use]
mod macros {
    //#[macro_export] //#[macro_export]  只用于 crate 根 想让宏被其他 crate 使用的情况；
    // 在同一 crate 内部用  #[macro_use]  即可。
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
