fn fib(n:u32)->u64{
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n{
        let  next = a+b;
        a = b;
        b = next;
    }
    a
}
//斐波那契数列中的第n个数是后面这个循环得到的

fn main(){
    let n = 10;
    let result  = fib(n);
    println!("斐波那契数列的第{}个数的值是{}",n,result);
}