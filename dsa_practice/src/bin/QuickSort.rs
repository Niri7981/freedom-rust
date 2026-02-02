#![allow(unused)]
pub fn partition(arr:&mut [i32],lo:usize,hi:usize)->usize{

    let pivot = arr[hi];
    let mut idx = lo;
   for i in lo..hi{
    if arr[i] <= pivot{
        arr.swap(i,idx);
        idx +=1;
    }
    arr.swap(idx,hi);
   }
   return idx;
}

fn qs(arr:&mut [i32],lo:usize,hi:usize){
    if lo >= hi{
    return;}
    
    let partition_idx = partition(arr,lo,hi);
    if partition_idx > 0{
    qs(arr,lo,partition_idx-1);
    }
    qs(arr,partition_idx+1,hi);
}

fn main(){
    let mut data = vec![8,2,4,4];
    let len = data.len();
    
    if len > 0 {
        // 调用你的 qs
        qs(&mut data, 0, len - 1);
    }
    
    println!("Sorted array: {:?}", data);
}

//终于git完事了，他妈的
// 🚀 Implemented QuickSort: O(n log n) speed with Rust safety!