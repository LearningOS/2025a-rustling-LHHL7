// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num   //(factorial:阶乘)
    // Do not use:
    // - return
    // Try not to use://不能用
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).product()//1..=num为 生成了从1到num的迭代器 1..num是从1到num-1
    //product()是标准库的迭代器方法  迭代器都可调用 作用：把迭代器里的所有元素 依次做乘法
    //返回类型根据上下文确定
    // let mut res=1;
    // let mut number=num;
    // //for一般遍历集合  while一般用于条件
    // while number !=0{
    //     res*=number;
    //     number-=1; 
    // }
    // res
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
