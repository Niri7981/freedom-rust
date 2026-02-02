struct TextProcessor<'a>{
    content:&'a str,
}

impl<'a> TextProcessor<'a>{
    fn get_summary(&self) -> &str {
        &self.content[0..2]
    }
}
 fn main(){
    {let x = TextProcessor{
        content:&String::from("yep"),
    };

    
    {
    let y = TextProcessor{
        content:&String::from("nope"),
    };

    let _the_first_two_x = x.get_summary();
    let _the_first_two_y = y.get_summary();

    }
  }
 }