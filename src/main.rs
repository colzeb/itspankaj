use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;

mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


async fn handle_database() {


    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    let connection = SqliteConnection::establish(&database_url).expect(&format!("Error while connecting {}", &database_url));

 use crate::schema::users::dsl::*;
 use models::UserNew;
 let new_user = UserNew {
    name: "pank",
    address: "another city",
    date_created: "today"
 };

 diesel::insert_into(users)
    .values(&new_user)
    .execute(&connection)
    .expect("failed to insert");

let resutl = users.select(name)
    .load::<String>(&connection)
    .expect("Error");

    println!("result {:?}", resutl)


}

#[get("/")]
async fn hello(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    
    let data = json!({
        "name": "Pankaj"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

handle_database();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "/home/pi/Projects/basic2/static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move|| {
        App::new()
        .app_data(handlebars_ref.clone())
        .service(hello)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
