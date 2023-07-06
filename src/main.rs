use std::env;
use std::process;
use minigrep::Config;


fn main(){

let vector:Vec<String> = env::args().collect();
let config = Config::build(&vector).unwrap_or_else(|err|{
        eprintln!("{err}");
        process::exit(1);
 });
  match minigrep::run(config){
    Err(error)=>{eprintln!("{error}"); process::exit(1)},  
    _=>(),
  };
 	

}
