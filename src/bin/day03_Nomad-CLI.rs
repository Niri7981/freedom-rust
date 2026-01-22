struct Wallet{
    address:String,
    balance:f64,
}

//写三个方法，初始化钱包的关联函数，存款方法，显示信息方法
impl Wallet{
    fn new_wallet(addr:String)->Wallet{
        Wallet{
            address:addr,
            balance:0.0,
        }
        }
    fn desposit(&mut self ,amount:f64){
        if amount>0.0{
            self.balance += amount;
            println!("成功存入{}$到{}账户中",amount,self.address);
        }
        else{
        println!("存入金额必须大于0!");
        }
    }
    fn withdraw(&mut self,amount:f64){
        if self.balance>amount{
            self.balance -= amount;
        }
        else{
            println!("fuck off yo thailand!");
        }

    }
        fn show_info(&self){
        println!("--- 游民钱包 ---");
        println!("地址: {}", self.address);
        println!("余额: ${:.2}", self.balance); // {:.2} 保留两位小数
        println!("----------------");
        }
    }
    

    fn main(){
        let mut my_wallet= Wallet::new_wallet(String::from("AUSTRA_DREAM"));
        my_wallet.withdraw(500.0);
        my_wallet.show_info();
    }









  