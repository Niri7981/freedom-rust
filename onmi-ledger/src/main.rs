pub struct Crypto {
    pub name: String,
    pub amount: f64,
    pub price: f64,
}

pub struct Physical {
    pub name: String,
    pub value: f64,
}


pub trait Asset{
    fn value_used(&self)->f64;
}

impl Asset for Crypto{
    fn value_used(&self)->f64{
        self.amount*self.price
    }
}

impl Asset for Physical{
    fn value_used(&self)->f64{
        self.value
    }
}
//创建一个仓库
pub struct Ledger{
    pub list:Vec<Box<dyn Asset>>,
}

//如何往仓库中存东西
impl Ledger{
    fn new()->Self{
        Ledger{
            list:Vec::new(),
        }
    }

    fn push(&mut self,item: Box<dyn Asset>){
        self.list.push(item);  
    }

    fn total_value(&self)->f64{
        let n = self.list.len();
        let mut value_init = 0.0;
        for i in 0..n{
            value_init+=self.list[i].value_used();
        }
        return value_init;
    }

}



fn main(){
    let my_crypto = Crypto{
        name:String::from("BTC"),
        amount:0.5,
        price:60000.0,
    };
    let my_house = Physical{
        name:String::from("House"),
        value:5000.0,
    };

   let box1 = Box::new(my_crypto);
   let box2 = Box::new(my_house);

   let mut my_ledger = Ledger::new();
   my_ledger.push(box1);
   my_ledger.push(box2);

    

    
    println!("我的总资产是{}dollars",my_ledger.total_value());

}