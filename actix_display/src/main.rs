// //standard actix code to get a server up and running
// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// async fn greet(req:HttpRequest) -> impl Responder{
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()>{
//     HttpServer::new(||{
//         println!("function is firing");
//         let app = App::new()
//         .route("/", web::get().to(greet))
//         .route("/{name}", web::get().to(greet));
//         return app

//     })
//     // the app has to be returned from the closure for the .bind and .run functions to be enacted
//     .bind("127.0.0.1:8000")?
//     .workers(3)
//     .run()
//     .await
// }

// /*
// used the actix framework to define a view that extracts data from
// the request. We then redefine our main function as an async with the
// macro from acrix-rt - without it hte program would crash since main fns 
// are not allowed to be async. 
// */

use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(number: i8) -> i8{
    println!("number {} is runing", number);
    let two_seconds = time::Duration::new(2,0);
    thread::sleep(two_seconds);
    return 2
}

fn main(){
    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| do_something);
}