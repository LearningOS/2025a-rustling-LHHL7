// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());//复制一份 并拿到所有权 相当于&str->String 可以修改了
    string("nice weather".into());//变成另外一个类型&str->String或者String->&str
    string(format!("Interpolation {}", "Station"));//format!  总是返回一个 新的  String 
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());//相当于借用切片  切掉空白 &str和string都返回&str
    string("Happy Monday!".to_string().replace("Mon", "Tues"));// &str.replace  给你新的  String ； String.replace  把自己改完再交出来（move）
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());//转小写 同上作用自身 都返回string
}
