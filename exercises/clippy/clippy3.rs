// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = Some(());//option<()>其中（）代表单元unit类型
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    // resize(0, 5)  会清空 Vec，并填充 5 到长度为 0（也就是什么都不做）。
    //则直接用clear就好
//关键是： resize  返回的是  () ，不是 Vec
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a,&mut value_b);//必须可变引用 而不是直接传值 
    // 若直接传值 所有权给了swap后面不能再用变量了
    //定义中的mut指以后可以有人来改变value_a    swap参数的mut指swap可以来改变value_a
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
