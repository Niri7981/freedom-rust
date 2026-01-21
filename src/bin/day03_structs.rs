#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用 impl 块定义方法，这是第 5 章的灵魂
impl Rectangle {
    // 这里的 &self 表示借用，不会发生所有权移动
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 再写一个关联函数，用于创建一个正方形
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 20, height: 50 };
    let sq = Rectangle::square(10);

    // 调用方法，这时候字段被“读取”了，警告自动消失
    println!("矩形的面积是: {}", rect1.area());
    println!("正方形的详情: {:?}", sq);
}