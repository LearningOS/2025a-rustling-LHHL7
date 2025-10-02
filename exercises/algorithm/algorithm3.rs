/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::mem::swap;
fn sort<T:std::cmp::PartialOrd+Ord>(array: &mut [T]){
	//TODO
    //冒泡
    // for x in (1..array.len()).rev(){//x从1到len-1 可用rev()逆序迭代器从len-1到1
    //     // let end =array.len()-x;//end从len-1 到 1
    //     for index in 0..x{//index从0到end-1
    //         if array[index]>array[index+1] {
    //             // swap(array[index],array[index+1]);
    //             array.swap(index,index+1);
    //         }
    //     }
    // }
    array.sort();//有自带sort
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}