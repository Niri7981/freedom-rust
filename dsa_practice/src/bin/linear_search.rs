fn linear_search(haystack: &[i32], needle: i32) -> bool {
    let length = haystack.len();

    // 重点：0..length 才是从 0 到长度的索引范围
    for i in 0..length {
        if haystack[i] == needle { // 记得用 ==
            return true; // 记得拼写 true
        }
    }
    false
}

fn main() {
    let my_data = vec![1, 3, 5, 7, 9]; // 建议小写 my_data
    let found = linear_search(&my_data, 7); // 传引用
    println!("Found 7? {}", found);
}