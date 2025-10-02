// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;
//cargo run （或  rustc  直接编译）时，只有 rustc 内置的 lint 会生效
//lint是编译器发出的警告或者提示
//Clippy：官方“加强版”lint 集合
 //cargo clippy  会加载上百条额外规则 来帮助improve代码
fn main() {
    let pi = f32::consts::PI;//标准库有pi
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
