use actix_web::{web, App, HttpResponse,HttpServer,Responder};
use std::io;


// configure route <1>
pub fn general_routes (cg: &mut web::ServiceConfig) {
    cfg.route("/health",web::get().to (health_check_handler));

}
// config handler <2>

pub async fn health_check_handler() ->impl Responder {
  HttpResponse::Ok().json("Hello .EzyTutoe is alive and kicking ")

}

// Instantiate and run the HTTP server
#[actix_rt::main]
 async fn main () -> io::Result<()> {
  let app = move || App::new().configure(general_routes);
 
// Start HTTP server <4>
HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

 }

