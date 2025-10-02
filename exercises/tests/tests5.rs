// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces（花括号）,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler（编译器 hands its trust of soundness （可靠性）of your
// code to yourself! If you cannot prove（证明 the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.



/// # Safety  （对调用方的要求
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {//unsafe告诉编译器：
//“这段代码的内存安全由我自己保证，你别管了
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.

    //SAFETY:address已经确保是有效的
    unsafe {//在unsafe块的花括号里，你可以做平时 Rust 不允许的底层操作
        //整数也不能直接转成 可变引用 要用裸指针
        let add=address as *mut u32;
        *add=0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        //Safety注释告诉内部人“为什么这里满足文档列出的条件，可以安全地进入  unsafe 
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        //引用 → 整数 必须“经停”裸指针  可变引用不能直接转整数
        assert!(t == 0xAABBCCDD);
    }
}
