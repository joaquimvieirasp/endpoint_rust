use actix_web::*;

mod routes;
use routes::ping::*;
use routes::info::*;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new( || {
        App::new()
        .route("/ping", web::get().to(ping))
        .route("/info", web::get().to(info))
    });

    let porta: i32 = 9091;

    let api = api.bind(format!("127.0.0.1:{}", porta))
    .expect("Não conseguiu conectar...");

    println!("conectado com sucesso \n http://localhost:{}/ping", porta );

    api.run()
    .await
}


