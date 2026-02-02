fn main() {
    // 使用绝对路径调用
    restaurant::front_of_house::hosting::add_to_waitlist();
    
    println!("成功通过二进制入口调用了库中的函数！");
}
