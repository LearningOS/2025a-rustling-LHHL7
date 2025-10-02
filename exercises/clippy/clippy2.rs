// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x)=option{//if let语句可以替代match语句
        res += x;
    }
    println!("{}", res);
}
