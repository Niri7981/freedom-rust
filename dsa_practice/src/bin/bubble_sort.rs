fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    for i in 0..n {
        // 每一轮把最大的那个“泡泡”推到最后
        for j in 0..n - i - 1 {
            if list[j] > list[j + 1] {
                // 使用 Rust 标准库最稳的交换方法
                list.swap(j, j + 1);
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