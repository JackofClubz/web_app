//standard actix code to get a server up and running
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match.info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_rt:main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))

    })
    .bind("127.0.0.1:800")?
    .run()
    .await
}

