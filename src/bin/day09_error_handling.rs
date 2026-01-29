use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file()->Result<String,io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    
}
fn main(){

    let result = read_username_from_file();
    match result {
    Ok(s) => println!("文件夹的用户名是: {}", s), // 成功了，打印里面的内容 s
    Err(e) => println!("读取出错了: {:?}", e),    // 失败了，打印错误信息

    }
}
