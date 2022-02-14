use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use sqlx::{Connection,  Pool, Row};
use sqlx::prelude::*;
use sqlx::postgres::{PgConnection};

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url =env::var("DATABASE_URL")
        .expect("DATABASE_URL not in .env file");

    let db_pool = PgPoolOptions::new()
        .connect("postgres://postgres:123456789rg@localhost:5432/course")
        .await
        .unwrap();
    //

    let course_rows = sqlx::query(
        r#"select id, teacher_id , name, time from course "#
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let mut course_list = vec![];
    for row in course_rows{
        course_list.push(Course{
            id:row.get(0),
            teacher_id:row.get(1),
            name: row.get(2),
            time: Some(chrono::NaiveDateTime::from(row.get(3))),
        })
    }
    println!("Courses = {:?}",course_list);
    Ok(())

}


