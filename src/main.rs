use std::env;
use std::process;
use std::fs;
use std::error::Error;
fn main(){

let vector:Vec<String> = env::args().collect();
let config = Config::build(&vector).unwrap_or_else(|err|{
        println!("Error cause:{err}");
        process::exit(1);
 });
 
   if let Err(e)=run(config){
	println!("{}",e);
 	process::exit(1);     
   }

}


fn run(config:Config)->Result<(),Box<dyn Error>>{
   let content = fs::read_to_string(config.file_path)?;
   println!("{content}");
   Ok(())
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
