use std::env;
use tera::Tera;
use web::db::{ post, establish_connection, query};
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    error,
    http::{header::ContentType, StatusCode},
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
    web as aweb, App, Error, HttpResponse, HttpServer, Result, HttpRequest, Responder
};
use web::utils::init_logger;
use log::info;
use std::collections::HashMap;



async fn main_page(tmpl: aweb::Data<tera::Tera>,req: HttpRequest) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = req.match_info().get("name") {
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        tmpl.render("index.html", &ctx)
            .map_err(
                |_| error::ErrorInternalServerError("Template Error")
            )?
        
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("name", "World");
        tmpl.render("index.html", &ctx)
            .map_err(
                |_| error::ErrorInternalServerError("Template Error")
            )?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>Hello</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    info!("logger initialized");
    HttpServer::new(
        || {
            let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"),"/static/**/*")).unwrap();
            App::new()
                .app_data(aweb::Data::new(tera))
                .service(aweb::resource("/{name}").route(aweb::get().to(main_page)))
                // .route("/", aweb::get().to(index))
                // .route("/{name}", aweb::get().to(main_page))
        }
    )
    .bind("127.0.0.1:8000")?
    .run()
    .await
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     help();
    //     return;
    // }

    // let subcommand = &args[2];
    // match subcommand.as_ref() {
    //     "post" => new_task(&args[3..]),
    //     "show" => show_task(&args[3..]),
    //     _ => help(),
    // }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("post: missing col");
        return;
    }

    let conn = establish_connection("data.db");
    post(&conn, &args[0],&args[1]);
}

fn show_task(args: &[String]) {
    if args.len() > 1 {
        println!("show: unexpected argument");
        return;
    }

    let conn = establish_connection("data.db");
    print!("=======Posts=======\n");
    print!("id\ttitle\t\tbody\n");
    for task in query(&conn){
        println!("{}\t{}\t{}",task.id,task.title, task.body)
    }

}
