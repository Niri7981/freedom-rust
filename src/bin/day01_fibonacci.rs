// 斐波那契逻辑函数
fn fib(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let next = a + b; // 删掉了 mut，因为每轮循环它都是全新的
        a = b;
        b = next;
    }
    a // 返回最终结果
}

// 程序的入口，没有它程序跑不起来
fn main() {
    let n = 10;
    let result = fib(n);
    println!("第 {n} 个斐波那契数是: {result}");
}