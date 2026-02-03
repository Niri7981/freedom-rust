// 定义单向链表的节点
// derive(Debug) 为了方便你调试
#[derive(Debug)]

pub struct Node<T>{
    pub value:T,
    pub next:Option<Box<Node<T>>>,
}
pub struct LinkedList<T>{
    head:Option<Box<Node<T>>>,
}

impl <T> LinkedList<T>{
    
   pub  fn new()->Self{
        LinkedList{ head:None}
    } 

    pub fn push(&mut self,value:T){
        let new_node = Box::new(Node{value,
            next:self.head.take(),
        });
        self.head=Some(new_node); 
    }
}

fn main(){
    
}