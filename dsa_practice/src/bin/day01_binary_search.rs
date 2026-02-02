fn binary_search(haystack:&[i32],needle:i32)-> bool {
    if haystack.is_empty(){
        return false;
    }

    let mut lo:usize = 0;
    let mut hi:usize = haystack.len();

    while lo<hi{

        let mid = lo+(hi-lo)/2;
        let value = haystack[mid];

        if value == needle{
            return true;
            }else if value<needle{
                lo = mid+1;
            }else{
                hi = mid;
            }
        }
        false
}

fn main(){
    let my_sorted_data = vec![2,5,8,12,16,18,20,23,38];
    let target = 23;

    println!("正在有序数组{:?}中查找：{}",my_sorted_data,target);

    let found = binary_search(&my_sorted_data,target);

    
    if found {
        println!("✅ 找到了！(用了 O(log N) 的时间)");
    } else {
        println!("❌ 没找到。");
    }
}
