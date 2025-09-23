// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]// #[derive(PartialEq, Debug)]  是 Rust 中最常见的语法糖之一，
// 它让编译器自动为你的类型生成  PartialEq  和  Debug  trait 的实现。
//可以调试和比较 可以直接写p1=p2这种比较
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value ==0 {
            Err(CreationError::Zero)
        }else if value <0 {
            Err(CreationError::Negative)
        }else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
        // Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
