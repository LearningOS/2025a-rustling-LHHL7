// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


//编译器采用三条规则来判断引用何时不需要明确的注解
//第一条规则是编译器为每一个引用参数都分配一个生命周期参数。
//第二条规则是如果只有一个输入生命周期参数，那么将它赋予给所有输出生命周期参数
//第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
// 说明这是个方法，那么所有输出生命周期参数被赋予 self 的生命周期。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {//此时有参数中有两个不同的引用 
// 返回值必须与其中较短的引用保持一致 若返回值存在时间比较短的引用长  
// 当较短的引用死了 返回值指向哪呢？此时明显不合理 所以要加生命周期'a约束
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
