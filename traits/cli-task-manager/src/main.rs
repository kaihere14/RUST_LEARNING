mod task;mod error;mod store;
use clap::{Parser,Subcommand};
use crate::store::delete_task;
use crate::store::load_tasks;
use crate::store::save_tasks;
use crate::store::toggle_complete_status;
use crate::task::Task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None,)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Add{
        title:String
    },
    Done{
        id:u32
    },
    List,
    Delete{
        id:u32
    }
}

fn main() {
   
    let parsed = Cli::parse();
    match  parsed.command {
       Commands::Add{title}=> {
        let mut pre_data = load_tasks().unwrap();
        pre_data.push(Task { id: pre_data.len() as u32 + 1, title:title, done: false });
        let _ = save_tasks(&pre_data).unwrap();
       },
       Commands::Done{id}=>{
        let data = toggle_complete_status(id).unwrap();
        println!("your task has been marked : {:?}",data);
       },
       Commands::List=>{
        let data = load_tasks().unwrap();
        println!("{:?}",data);
       },
       Commands::Delete{id}=>{
        println!("your passed string is : {}",id);
        let data = delete_task(id);
        match  data {
            Ok(contents) => {
                println!("Your all tasks for now : {:?}",contents);
            },
            Err(e) => {
                println!("You have an error while Delete : {:?}",e )
            }
        }
       },
    }
}