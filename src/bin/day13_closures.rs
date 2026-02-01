use std::collections::HashMap;
struct Cacher<T>
    where T:Fn(u32)-> u32{
        calculation:T,
        value:HashMap<u32,u32>,
    }
impl<T> Cacher<T>
   where T:Fn(u32)-> u32{
    fn new(calculation:T)->Cacher<T>{
        Cacher{
            calculation,
            value:HashMap::new(),
        }
    }

    fn value(&mut self,arg:u32)->u32{
        let f = &self.calculation;
        *self.value.entry(arg)
        .or_insert_with(||f(arg))
        }
    }

    fn main() {
    let mut c = Cacher::new(|a| {
        println!("正在进行昂贵的计算...");
        a
    });

    let v1 = c.value(1);
    let v2 = c.value(1); // 这一次应该不会打印“正在进行昂贵的计算”
    let v3 = c.value(2); // 换了新数字，应该会再次打印

    println!("v1: {}, v2: {}, v3: {}", v1, v2, v3);
}