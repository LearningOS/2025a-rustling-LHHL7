/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
//是一个保证非空裸指针的封装
use std::vec::*;

#[derive(Debug)]//属性宏 
//让结构体 枚举等直接输出  println!("{:?}", p); // 输出：Point { x: 1, y: 2 }
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {//用类型名调用，参数列表里没有  self 。是关联函数
        //Default::default()类型调用
        //若参数列表有self是实例调用  a.default()
        Self::new()//下方的new
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
    pub fn add(&mut self, obj: T) {//链表尾部加一个为obj值的节点
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        //先将node变成裸指针  再套一层NonNull且是跳过空指针检查 由于Box永远非空
        //因为构造box时 空了就panic了
        //NonNull::new_unchecked本身算unsafe函数 所以要写入unsafe块里
        //再套一层some  则node_ptr:Option<NonNull<Node<T>>>
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }//end_ptr:NonNull<Node<T>>  as_ptr()也是拿裸指针
        //*end_ptr.as_ptr():Node<T>
        self.end = node_ptr;
        self.length += 1;
    }
}
impl<T: std::cmp::PartialOrd+Copy> LinkedList<T> {
    

    pub fn get(&mut self, index: i32) -> Option<&T> {//得到第index个节点的值引用 从0开始
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {//得到node的往后的第index个节点
            None => None,//若传入空节点则返回空
            Some(next_ptr) => match index {//若index为0则返回node的值引用
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
                //不然递归传入node的下一个节点和index-1
            },
        }
    }
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
	{
		//TODO
        let sum_length=list_a.length+list_b.length;
        if sum_length==0{//先处理两个链表均空的情况
            return Self {
            length: 0,
            start: None,
            end: None,
        }
        }
        let mut res= LinkedList{
            length:sum_length,
            start:None,
            end:None,
        };
        let (mut index_a,mut index_b)=(0,0);
        while index_a<list_a.length&&index_b<list_b.length {//处理两个链表还未空时
            let value_a=list_a.get(index_a as i32).unwrap();
            let value_b=list_b.get(index_b as i32).unwrap();
            if *value_a <= *value_b {//a链表此时小
                res.add(*value_a);
                index_a+=1;
            }else {//b链表此时小
                res.add(*value_b);
                index_b+=1;
            }
        }
        //此时有链表为空了 无法比较了
		if index_a>=list_a.length {//a空  则把b剩下的加进来
            while index_b<list_b.length {
                let value_b=list_b.get(index_b as i32).unwrap();
                res.add(*value_b);
                index_b+=1;
            }
        }
        if index_b>=list_b.length {//b空  则把a剩下的加进来
            while index_a<list_a.length {
                let value_a=list_a.get(index_a as i32).unwrap();
                res.add(*value_a);
                index_a+=1;
            }
        }
        res
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}