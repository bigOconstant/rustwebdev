
use actix_web::{web};
use crate::template_logic::register::{index,loggedin,register,home};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
    .service(web::resource("/").route(web::get().to(home)))
    .service(
        web::resource("/register")
        .route(web::post().to(register))
        .route(web::get().to(index)
        ))
    .service(
        web::resource("/loggedin").route(web::get().to(loggedin))
    ));
}
