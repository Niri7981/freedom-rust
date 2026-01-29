pub struct Node<T>{
    pub value:T,
    pub next:Option<Box<Node<T>>>
}

pub struct MyQueue<T>{
    pub head:Option<Box<Node<T>>>,
    pub length:usize,
}

impl<T> MyQueue<T>{
     pub fn new()->Self{
        MyQueue{head:None,length:0,}
     }
     
     pub fn enqueue(&mut self,item:T){

        let new_node = Box::new(Node{
            value:item,
            next:None,
        });

        self.length+=1;

        if self.head.is_none(){
            self.head = Some(new_node);
        }
        else{
            let mut curr = self.head.as_mut().unwrap();
            while curr.next.is_some(){
                curr = curr.next.as_mut().unwrap();
            }
            curr.next = Some(new_node);
        }

     }


     pub fn dequeue(&mut self)->Option<T>{
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
fn test_queue_with_strings() {
    let mut q = MyQueue::new();
    q.enqueue("BTC".to_string());
    q.enqueue("ETH".to_string());

    assert_eq!(q.dequeue(), Some("BTC".to_string()));
}
}
