// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated{注解}
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {//本身函数只需引用就好 不要所有权 
// 若改成string违反设计初衷
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    
    {
        let result;
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);//此时改作用域就好
    }
    
}
