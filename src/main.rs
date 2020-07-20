use tokio::time;
use std::time::Duration;
use futures::executor::block_on;
use std::process::{Command};
use clap::App;



#[tokio::main]
async fn main() {

     loop_schedule().await;
}

async fn loop_schedule(){
    let matches = App::new("rust_schedule")
        .version("1.0")
        .author("robust. <ucgygah@gmail.com>")
        .about("Does awesome things")
        .arg("-f, --file=[FILE] 'Sets a shell command file which include full path'")
        .arg("-t  --time=[TIME] 'Sets a period time,the unit is ms'")
        .get_matches();
        let mut command_name = "dir";   
    if let Some(o) = matches.value_of("file") {
        println!("Value for FILE: {}", o);
        command_name = o
    }
    let mut   interval_time :u64 = 30000;

    if let Some(o) = matches.value_of("time") {
        println!("Value for TIME: {}", o);
        interval_time = o.parse().unwrap()
    }

    println!("command_name:{}, time:{}",command_name,interval_time);

   
    let mut interval = time::interval(Duration::from_millis(interval_time));
    loop {
        interval.tick().await;
        schedule_start(command_name.to_string());
    }
}

fn schedule_start(command_str:String){
    block_on(exec(command_str));
}
async fn exec(command_str:String){
    let result  = 
    Command::new(&command_str)
            .output()
            .expect("failed to execute process");
    println!("the command '{}', run result :{:?}",command_str,result);

}


