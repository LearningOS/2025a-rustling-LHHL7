// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



//可以定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解。
struct Book<'a> {//如果实例存活的更久 而结构体变量是引用 且更早死了 那么实例的这个变量就指向的有问题了
    //Rust 会拒绝这段代码——它根本编译不过 
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
