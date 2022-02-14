use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::models::Course;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use actix_web::web;
    use crate::AppState;
    use crate::models::Course;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not in .env");
        println!("{}",database_url);
        let db_pool = PgPoolOptions::new()
            .connect(&database_url.to_string())
            .await
            .unwrap();
        let app_state = web::Data::new(AppState{
            health_check_response:"I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db:db_pool,
        });
        let course = web::Json(Course{
            teacher_id:1,
            name:"11".into(),
            id:None,
            time:None,
        });
        let res = new_course(course,app_state).await;
        assert_eq!(res.status(),StatusCode::OK);
    }
}

