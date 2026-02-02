pub struct Node<T>{
    pub value:T,
    pub next:Option<Box<Node<T>>>,
}

pub struct MyStack<T>{
    pub head:Option<Box<Node<T>>>,
    pub length:usize,
}

impl <T> MyStack <T> {
    pub fn new()->Self{
        MyStack{
            head:None,
            length : 0,
        }
    }

    pub fn push(&mut self,item:T){
        let new_node = Box::new(Node{
            value:item,
            next:self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) ->Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            self.length-=1;
            node.value
        })
    }
}

fn main(){
    }

#[cfg(test)]
mod tests{
    use super::*;
 #[test]

fn test_with_123(){
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
}



}



