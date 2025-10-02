// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]//#[test]：这个属性表明这是一个测试函数
    fn you_can_assert() {
        assert!(true);//assert! 宏来断言  接受bool参数
        //assert_eq! 和 assert_ne!是比较两个参数
    }
}
