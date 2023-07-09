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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(page_index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
