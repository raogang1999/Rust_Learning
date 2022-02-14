use super::handlers::*;
use actix_web::{route, web};
//get health info
pub fn general_routes(cfg:&mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}
//add courses
pub fn course_routes(cfg:&mut web::ServiceConfig){
    cfg
        .service(web::scope("/courses"))
        .route("/",web::post().to(new_course))
        .route("/{user_id}",web::get().to(get_courses_for_teacher))
        .route("/{user_id}/{course_id}",web::get().to(get_course_detail));
}