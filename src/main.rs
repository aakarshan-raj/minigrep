use std::env;
use std::process;
use minigrep::Config;


fn main(){

let vector:Vec<String> = env::args().collect();
let config = Config::build(&vector).unwrap_or_else(|err|{
        process::exit(1);
 });
 
   if let Err(e)=minigrep::run(config){
 	process::exit(1);     
   }

}
