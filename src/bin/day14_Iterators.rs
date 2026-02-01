#[derive(PartialEq,Debug)]
struct Counter {
    count:u32,
}
impl Counter{
    fn new() ->Counter{
        Counter{
            count:0,
        }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self)->Option<Self::Item>{
        self.count += 1;

        if self.count<6{
            Some(self.count)
        } else {
            None
        }
    }
}



fn main(){
    let v1 = vec![1,2,3];
    let v2 = vec![4,2,1];
    let sum:Vec<_> = v1.iter().zip(v2.iter()).map(|(a,b)| a+b).collect();

    println!("{:?}",sum);
}