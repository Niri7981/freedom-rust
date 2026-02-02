#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{

    
    fn area(&self) ->u32{
        self.width * self.height
    }



    fn squre(size:u32)->Rectangle{
       Rectangle{
        width:size,
        height:size,
       }
    }

}


fn main() {
    let rect1 = Rectangle{
        width:20,
        height:10
    };
        
    let sq = Rectangle::squre(10);
    
   
    println!("矩形的面积是: {}", rect1.area());
    println!("正方形的详情: {:?}", sq);
}