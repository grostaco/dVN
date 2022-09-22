use log4rs::{append::{console::ConsoleAppender, file::FileAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root, Logger}};
use rocket::{fs::FileServer};

mod routes;
use routes::api::{clear_rendered, image_preview, render};

#[macro_use] extern crate rocket;

// pub struct CORS;

// #[rocket::async_trait]
// impl Fairing for CORS {
//     fn info(&self) -> Info {
//         Info {
//             name: "Add CORS headers to responses",
//             kind: Kind::Response,
//         }
//     }

//     async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Methods", "GET, PUT, POST, DELETE, OPTIONS"));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//     }
// }

// This program should be run with cargo -p
#[launch]
fn rocket() -> _ {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d} {l}  {M}] {m}\n")))
        .append(false)
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("image_rpg", log::LevelFilter::Debug))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();

    rocket::build()
    .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/api", routes![image_preview, render, clear_rendered])
}