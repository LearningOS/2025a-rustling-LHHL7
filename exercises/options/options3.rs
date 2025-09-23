// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // if let Some(p)=y{
    //     println!("Co-ordinates are {},{} ", p.x, p.y);
    // }else{
    //     panic!("no match!");
    // }
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),//如果  y  是  Some(value) ，那么  value  的所有权会移动给  p ， y  此后不能再被使用
        _ => panic!("no match!"),//必须给p前面加ref 只是借用
        //ref - 通过引用绑定  而不是按值绑定  reference 的缩写写法
        //模式匹配时都是默认按值绑定
    }
    y; // Fix without deleting this line.
    //在不删除这行的前提下修复
}
