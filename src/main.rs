use std::env;

struct Config{

  query:String,
  file_path:String,

}

fn main() {
  let v:Vec<String> = env::args().collect();
  let config = parse_config(&v); 

}

fn parse_config(v:&Vec<String>)->Config{

  let a = v[1].clone();
  let b = v[2].clone();
  
  Config{query:a,file_path:b}

}
