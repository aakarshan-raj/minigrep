use std::env;
use std::process;

fn main(){

let vector:Vec<String> = env::args().collect();
let config = Config::build(&vector).unwrap_or_else(|err|{

        println!("Error cause:{err}");
        process::exit(1);

 });

println!("search for:{} in: {}",config.query,config.file_path);

}


struct Config{

query:String,
file_path:String,

}

impl Config{

   fn build(vector:&Vec<String>)->Result<Config,&'static str>{

       if vector.len() < 3 {
         return Err("Argument error");
       }
       let query = vector[1].clone();
       let file_path = vector[2].clone();
       Ok(Config{query,file_path})

   }

}
