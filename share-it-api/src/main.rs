use actix_web::{web, HttpServer, App, Error as AWError, HttpResponse, Result};
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let addr = match std::env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0:8000".to_string(),
    };

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/login")
                    .route(web::post().to_async(login))
            )
            .service(
                web::resource("/chatroom/{id}")
                    .route(web::get().to_async(enter_chatroom))
            )
    })
        .bind(&addr)?
        .run()
        .await
}