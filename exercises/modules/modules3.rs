// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.



// TODO: Complete this use statement
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        //panic!()是Rust 的“紧急刹车”宏
        //宏指这段代码 被编译完后 成了另一段代码
        //一旦执行，当前线程立即终止回退，后打印你给出的消息并退出程序
    }
}
