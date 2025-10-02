/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
    //约定（惯例）：
//返回  true  表示第一个参数比第二个“更优先”（即应排在前面）。
//用途：
//构建小顶堆时传  |a, b| a < b 
//构建大顶堆时传  |a, b| a > b
//let min_heap: Heap<i32> = Heap::new(|a, b| a < b);
//let max_heap: Heap<i32> = Heap::new(|a, b| a > b);
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
//创建了一个长度为 1 的  Vec<T> 
 //内部只包含 一个元素，这个元素是  T::default()  —— 也就是  T  的默认值。
 //则我自己的元素索引应该从1开始
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count+=1;
        if self.count>=self.items.len() {
            self.items.push(value);
        }else{
        self.items[self.count]=value;//先加入最底下 然后不断上升
        }
        let mut idx=self.count;//新元素的idx
        
        //与父节点比较 若优先则上升
        while idx>1{
            let parent=self.parent_idx(idx);
        if (self.comparator)(&self.items[idx],&self.items[parent]){
            self.items.swap(idx,parent);
            idx=parent;//更新位置
        }
        else{ break}
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		if self.children_present(idx){
            if (idx*2+1) <= self.count{//两个孩子都有
                let left=idx*2;
                let right=idx*2+1;
                if (self.comparator)(&self.items[left],&self.items[right]){
                    return left
                }else {
                    return right
                }
            }
            //只有左
            return idx*2
        }
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count==0 {
            return None
        }
        // std::mem::take(&mut T) -> T 
//把可变引用里的值“掏”出来，同时在原位置放入  T::default() ，最后把掏出的值返回给你。
        let top=std::mem::take(&mut self.items[1]);//取堆顶
        self.items[1]=std::mem::take(&mut self.items[self.count]);//取最底下元素
    //     let tail = self.items.pop().unwrap();
    // self.items[1] = tail;
        self.count-=1;
        // 再放到堆顶
        //调整堆  下沉
        let mut idx=1;
        while self.children_present(idx) {
            let left=self.left_child_idx(idx);
            let right=self.right_child_idx(idx);
            // if right<self.count{//右孩子也存在
            let small=self.smallest_child_idx(idx);
            if(self.comparator)(&self.items[small],&self.items[idx]){
                    self.items.swap(idx,small);
                    idx=small;
            }else { break }//比孩子优先 就不用动
                // if (self.comparator)(&self.items[left],&self.items[right]){//若左优先
                //     if(self.comparator)(&self.items[left],&self.items[idx]){
                //         self.items.swap(idx,left);
                //         idx=left;
                //     }else { break }//比孩子优先 就不用动
                // }else{
                //     if(self.comparator)(&self.items[right],&self.items[idx]){
                //         self.items.swap(idx,right);
                //         idx=right;
                //     }else { break }//比孩子优先 就不用动
                // }
            // }else{//只有左孩子
            //     if(self.comparator)(&self.items[left],&self.items[idx]){
            //             self.items.swap(idx,left);
            //             idx=left;
            //         }else { break }//比孩子优先 就不用动
            // }
        }
        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}