#[derive(Debug)]
enum Destination{
    Domestic(String),
    International{country:String,currency:String},
    Planning,
}
fn check_travel_budget(dest:Option<Destination>){
    match dest {
        Some(Destination::Domestic(city))=>{
           println!("正在去往 {} 的路上，数字游民第一步！", city);
        }

        Some(Destination::International { country, currency }) => {
            println!("准备前往 {}，记得兑换一些 {}。", country, currency);
        }

        
        Some(Destination::Planning) => {
            println!("正在研究攻略，环游世界指日可待。");
        }
        None => {
            println!("目前没有旅行计划，先在哈尔滨好好健身！");
        }
    }
}

fn main(){
    let my_next_step = Some(Destination::International {
        country: String::from("Australia"),
        currency: String::from("AUD"),
    });

    check_travel_budget(my_next_step);
    let _planning = Destination::Planning; 
    let _domestic = Destination::Domestic(String::from("Harbin"));
}