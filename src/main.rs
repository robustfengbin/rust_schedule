use tokio::time;
use std::time::Duration;
use futures::executor::block_on;
use std::process::{Command};

use std::env;

const INTERVAL_TIME:u64 = 3000;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut command_name = "dir";
    if args.len()>1{
        println!("the args len greater than one the fuck:{}",args.len());
         command_name = &args[1];
    }
    let mut interval = time::interval(Duration::from_millis(INTERVAL_TIME));
    loop {
        interval.tick().await;
        schedual_start_eth(command_name.to_string());
    }
}
fn schedual_start_eth(command_str:String){
    block_on(exec(command_str));
}
async fn exec(command_str:String){
    let result  = 
    Command::new(&command_str)
            .output()
            .expect("failed to execute process");
    println!("the command '{}', run result :{:?}",command_str,result);

}

