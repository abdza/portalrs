use tera::{Tera, Context};
use actix_web::{get, post, web, error,  App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;

macro_rules! render {
    ($template:expr, $context:expr) => {
        {
            match TEMPLATES.render($template, &$context) {
                Ok(body) => Ok(HttpResponse::Ok().body(body)),
                Err(err) => {
                    eprintln!("## Tera error: {}", err);
                    Err(error::ErrorInternalServerError(err))
                },
            }
        }
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[get("/")]
async fn hello() -> impl Responder {
    let ctx = Context::new();
    render!("index.html",ctx)
}

#[get("/pages/")]
async fn page_index() -> impl Responder {
    let ctx = Context::new();
    render!("pages/index.html",ctx)
}

#[get("/pages/create")]
async fn page_create() -> impl Responder {
    let ctx = Context::new();
    render!("pages/form.html",ctx)
}

#[post("/pages/save")]
async fn page_save() -> impl Responder {
    web::Redirect::to("/pages/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(page_index)
            .service(page_create)
            .service(page_save)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
