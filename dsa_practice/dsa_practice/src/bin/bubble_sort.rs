fn bubble_sort<T:PartialOrd>(list:&mut [T]){
    let n = list.len();
    for i in 0..n{
        for j in 0..n-i-1{
            if &list[j]>&list[j+1]{
                list.swap(j,j+1);
            }
        }
    }
    
}
   

fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", arr);

    bubble_sort(&mut arr);

    println!("排序后: {:?}", arr);
}