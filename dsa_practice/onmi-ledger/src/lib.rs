pub fn bubble_sort<T:PartialOrd>(list:&mut [T]){
    let n = list.len();
    for i in 0..n{
        for j in 0..n-i-1{
            if &list[j]>&list[j+1]{
                list.swap(j,j+1);
            }
        }
    }
    
}
   

