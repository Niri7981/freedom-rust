use  std::alloc::{self,Layout};
use  std::ptr::copy_nonoverlapping;//  搬家操作
use  std::alloc::dealloc;
use std::ptr::read;
pub struct ArrayList<T>{
    pub pointer:*mut T,
    pub len:usize,
    pub cap:usize,
}

impl<T>ArrayList<T>{
    pub fn new()->Self{
        let cap = 2;

        let layout = Layout::array::<T>(cap).expect("计算内存布局失败");

        let ptr = unsafe{   
        let raw_ptr = alloc::alloc(layout);
            raw_ptr as *mut T
        };
        if ptr.is_null(){
            std::alloc::handle_alloc_error(layout);
        }

        Self{
            pointer:ptr,
            len:0,
            cap:cap,
        }  
     }

     pub fn grow(&mut self){
        let new_cap = self.cap*2;

        let new_layout = Layout::array::<T>(new_cap).expect("新图纸设计失败");

        let new_ptr = unsafe{
            alloc::alloc(new_layout) as *mut T
        };

        if new_ptr.is_null(){
            alloc::handle_alloc_error(new_layout);
        }
        unsafe{
            copy_nonoverlapping(self.pointer,new_ptr,self.len);
        }

        let old_layout = Layout::array::<T>(self.cap).unwrap();

        unsafe{dealloc(self.pointer as *mut u8,old_layout)};
        self.pointer = new_ptr;
        self.cap = new_cap;

     }


     pub fn push(&mut self,item:T){
        if self.len == self.cap{
            self.grow();
        }
        unsafe{
        let target_address = self.pointer.add(self.len);

        std::ptr::write(target_address,item);
     }

     self.len +=1;
     }

     pub fn get(&self,index:usize)->Option<&T>{
        if index<self.len{
            unsafe{
                let p = self.pointer.add(index);
                Some(&*p)
            }
        }else{
            None
        }
     }

     pub fn pop(&mut self)->Option<T>{
        if self.len == 0{
            None
        }else{
            self.len -=1;
            unsafe{
                let last_ptr = self.pointer.add(self.len);
                let value  = read(last_ptr);
                Some(value)
            }
        }
     }
}

impl<T> Drop for ArrayList<T>{
    fn drop(&mut self){
        if!self.pointer.is_null(){
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                dealloc(self.pointer as *mut u8,layout);
            }
        }      
    }
}

fn main(){
    let mut my_array_list = ArrayList::new();
    my_array_list.push(100);
    my_array_list.push(200);

    println!("弹出了: {:?}", my_array_list.pop()); 
    println!("剩下的第一个: {:?}", my_array_list.get(0)); 
}




