fn main(){
    let s1 = String::from("Hello,RUST");
    let s2 = s1;

    let len = calculate_length(&s2);
    println!("The length of {} is {}",s2,len);
    println!("{}",s2);

    let mut s3 = String::from("Digital");
    change(&mut s3);
    println!("I AM{}",s3);    
}

fn calculate_length(s:&String)->usize{
    s.len()
}

fn change(some_string:&mut String){
    some_string.push_str("Nomad");
}