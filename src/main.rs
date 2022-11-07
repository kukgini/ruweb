use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short, long, default_value = "9020")]
    port: u32,
    #[clap(short, long, default_value = "false")]
    interactive: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    let opts: Opts = Opts::parse();
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
    })
    .keep_alive(std::time::Duration::from_secs(30))
    .client_request_timeout(std::time::Duration::from_secs(30))
    .workers(1)
    .bind(format!("{}:{}", host, opts.port))?
    .run()
    .await
}
