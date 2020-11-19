mod controllers;
use controllers::routers::server_run;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    server_run()
}
