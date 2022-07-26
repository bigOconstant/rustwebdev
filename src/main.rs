use actix_web::{ middleware, web, App, HttpServer};
use tera::Tera;
use std::env;

#[macro_use]
extern crate diesel;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
  };

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder()

    .max_size(5)// To do, put this in a config somewhere
    .build(manager)
    .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));


    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Listening on: localhost:8080, open browser and visit have a try!");
    HttpServer::new( move|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(spacial_learning::routes::routes)
            .service(web::scope("").wrap(spacial_learning::template_logic::errors::error_handlers()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

