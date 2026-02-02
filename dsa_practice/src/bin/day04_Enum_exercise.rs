enum GymMembership {
    Silver,
    Gold(String), // 金卡会员有个专属教练的名字
    Diamond { months: u32, personal_trainer: bool }, // 钻石卡有有效期和是否含私教
}

fn check_membership(m: GymMembership) {
    match m {
        GymMembership::Silver => println!("普通撸铁区欢迎你。"),
        // 【练习 1】：请补充 Gold 分支，打印出教练的名字
        GymMembership::Gold(name)=>println!("{}",name),
        __________________________________________________
        // 【练习 2】：请补充 Diamond 分支，如果 months > 12，打印“核心会员”
        GymMembership::Diamond{months..}=>if months > 12 {println!("central vip")},__________________________________________________
    }
}