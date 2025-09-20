// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.


struct ColorClassicStruct {
    // TODO: Something goes here
    red: usize,
    green: usize,
    blue: usize,
}

struct ColorTupleStruct(usize,usize,usize/* TODO: Something goes here */);//元组结构体：给元组起名字 但每个字段不要名字

#[derive(Debug)]//配合{:?}帮助我格式化打印结构体/枚举（还没学）
struct UnitLikeStruct;//没有任何字段的类单元结构体

#[cfg(test)]//一个测试模块
mod tests {
    use super::*;//测试模块的固定语句

    #[test]//将一个函数变成测试函数
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green= ColorClassicStruct {
            red:0,
            green:255,
            blue:0,
        };

        assert_eq!(green.red, 0);//对结果进行断言
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green=ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let unit_like_struct=UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);//format格式化打印  变量外面加上花括号会被一整个替换
        
        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
