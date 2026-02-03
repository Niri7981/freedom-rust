use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::fmt::Display;



struct Node<T>{
    value:T,
    next:Option<Rc<RefCell<Node<T>>>>,
    prev:Option<Weak<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T>{
    head:Option<Rc<RefCell<Node<T>>>>,
    tail:Option<Rc<RefCell<Node<T>>>>,
    length:usize,
}

impl<T: Display> DoublyLinkedList<T>{
   pub  fn new()->Self{
        DoublyLinkedList{
            head:None,
            tail:None,
            length:0,
        }
    }

    pub fn push_front(&mut self ,value:T){
        let new_node = Rc::new(RefCell::new(Node{
            value,
            next:None,
            prev:None,
        }));
        
        match self.head.take(){
            Some(old_head) => {
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
    }
        self.length +=1;
    }

    pub fn display(&self){
        let mut curr = self.head.clone();
        print!("LinkedList:");
        while let Some(node) = curr{
            print!("{}->",node.borrow().value);
            curr = node.borrow().next.clone();
        }
        println!("None");
    }

    pub fn push_back(&mut self,value:T){
        let new_node = Rc::new(RefCell::new(Node{
            value,
            next:None,
            prev:None,
        }));

        match self.tail.take(){
            Some(old_tail)=>{
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None=>{
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    pub fn pop_back(&mut self) ->Option<T>{
        let old_tail = self.tail.take()?;
        let weak_prev = old_tail.borrow().prev.clone();

        match weak_prev{
            Some(weak) =>{
                if let Some(new_tail) = weak.upgrade(){
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
            }
            None =>{
                self.head = None;
            }
        }
        self.length -=1;
        Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value)
    }

    pub fn pop_front(&mut self)->Option<T>{
        let old_head = self.head.take()?;
        let rc_next = old_head.borrow().next.clone();

        match rc_next{
            Some(new_head)=>{
                new_head.borrow_mut().prev = None;

                self.head = Some(new_head.clone());              
            }
            None=>{
                self.tail = None;
            }
        }
        self.length -= 1;
        Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().value)
    }
}

pub struct Stack<T>{
    list:DoublyLinkedList<T>,
}

impl<T:Display>Stack<T>{
    pub fn new()->Self{
        Stack{
            list:DoublyLinkedList::new()
        }
    }
    pub fn push(&mut self,item:T){
        self.list.push_back(item);
    }
    pub fn pop(&mut self)->Option<T>{
        self.list.pop_back()
    }
}



pub struct Queue<T>{
    list:DoublyLinkedList<T>,
}

impl<T: std::fmt::Display> Queue<T> {
    pub fn new() -> Self {
        Queue { list: DoublyLinkedList::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.list.push_back(item); // 进队
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()      // 出队 
    }
}



fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack pop: {:?}", stack.pop().unwrap()); // 应该是 3 (后进先出)

    // 测试队列
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("Queue dequeue: {:?}", queue.dequeue().unwrap()); // 应该是 1 (先进先出)
}
