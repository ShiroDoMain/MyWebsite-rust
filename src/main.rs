use std::env;
use web::db::{ post, establish_connection, query};

fn help(){
    println!("subcommands:");
    println!("    post<title> <body>: post a new data")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        help();
        return;
    }

    let subcommand = &args[2];
    match subcommand.as_ref() {
        "post" => new_task(&args[3..]),
        "show" => show_task(&args[3..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("post: missing col");
        help();
        return;
    }

    let conn = establish_connection("data.db");
    post(&conn, &args[0],&args[1]);
}

fn show_task(args: &[String]) {
    if args.len() > 1 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection("data.db");
    print!("=======Posts=======\n");
    print!("id\ttitle\t\tbody\n");
    for task in query(&conn){
        println!("{}\t{}\t{}",task.id,task.title, task.body)
    }

}
