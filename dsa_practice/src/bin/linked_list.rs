// 定义单向链表的节点
// derive(Debug) 为了方便你调试
#[derive(Debug)]


pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>, // 重点：用 Box 包裹，解决递归类型大小未知的问题
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

fn main(){
    
}