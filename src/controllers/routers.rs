use actix_web::{middleware, web, App, HttpServer};
use tera::Tera;

use crate::controllers::{
    home_controller::HomeController,
    user_controller::UserController,
    render_controller::RenderController,
    error_controller::ErrorController,
    preludes::Controller
};

#[actix_web::main]
pub async fn server_run() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(get!("/", HomeController::index))
            .service(get!("/{html_name}", RenderController::index))
            .service(get!("/user", UserController::index))
            .service(scope!("", ErrorController::error_handlers()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
