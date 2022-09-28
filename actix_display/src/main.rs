//standard actix code to get a server up and running
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match.info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_rt:main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        println!("function is firing");
        let app = App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet));
        return app

    })
    // the app has to be returned from the closure for the .bind and .run functions to be enacted
    .bind("127.0.0.1:800")?
    .run()
    .await
}

/*
used the actix framework to define a view that extracts data from
the request. We then redefine our main function as an async with the
macro from acrix-rt - without it hte program would crash since main fns 
are not allowed to be async. 
*/