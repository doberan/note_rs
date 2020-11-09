mod controllers;

fn main() -> std::io::Result<()> {
    controllers::routers::server_run()
}
