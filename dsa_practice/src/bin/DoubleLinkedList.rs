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




fn main() {
    // 🧪 测试 1: 存字符串 (String)
    let mut str_list = DoublyLinkedList::new();
    str_list.push_back("Hello"); // 自动推导 T = &str
    str_list.push_back("World");
    str_list.push_front("Rust");
    println!("--- String List ---");
    str_list.display(); // 应该打印: Rust -> Hello -> World -> None

    // 🧪 测试 2: 存整数 (i32)
    let mut int_list = DoublyLinkedList::new();
    int_list.push_back(100);
    int_list.push_back(200);
    println!("--- Int List ---");
    int_list.display(); // 应该打印: 100 -> 200 -> None
}