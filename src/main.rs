use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Serialize;
use tera::Tera;

mod time_service;
use futures::future::{ready, Ready};
use time_service::TimeInfo;

#[derive(Serialize)]
struct TemplateContext {
    time_str: String,
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("time_str", &time_service::get_random_time().random_time);
    let body = tmpl
        .render("index.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn api_current() -> impl Responder {
    let current_info = time_service::get_random_time();
    current_info
}

impl Responder for TimeInfo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(tera)
            .service(actix_files::Files::new("/public", "./static"))
            .route("/", web::get().to(index))
            .route("/api/current", web::get().to(api_current))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
