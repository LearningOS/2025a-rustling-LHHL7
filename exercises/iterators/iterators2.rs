// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();// chars()  返回的是一个 迭代器 吐出每一个字符
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect()
        //单个字符调用to_uppercase()时 因为有些字符转大写后不止一个char 所以返回一个迭代器
        //chain(c)  就是把两个迭代器串成一根“长”迭代器，
        //collect将迭代器直接变成集合 目标类型必须由上下文推断；这里函数签名要求返回  String ，Rust 会 自动 选择  String
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut v:Vec<String> =Vec::new();
    let  w_iter=words.iter();
    for x in w_iter {
            v.push(capitalize_first(&x));
    }
    v
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut res=String::new();
    for x in words.iter(){
        res.push_str(&capitalize_first(&x));
    }
    res

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
