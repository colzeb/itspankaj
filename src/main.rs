use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

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
