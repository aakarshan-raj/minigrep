use std::fs::File;
use std::io::{self,Read};


fn main() {
    let data = read_the_file("poemdsf.txt");
    println!("{data}");
}

fn read_the_file(file_name:&str)->String{

 let mut x = match File::open(file_name){
      Ok(yes)=>yes,
      Err(e)=>panic!("{e}"),
   };
 let mut username = String::new();
 let data = x.read_to_string(&mut username);
 return username;

}
