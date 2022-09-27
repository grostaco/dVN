use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use rocket::fs::FileServer;

mod routes;
use routes::api::*;

#[macro_use]
extern crate rocket;

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
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("image_rpg", log::LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    rocket::build()
        .mount("/", FileServer::from("backend/static"))
        .mount("/assets", FileServer::from("assets").rank(1))
        .mount(
            "/api",
            routes![
                render::image_preview,
                render::render,
                render::clear_rendered,
                file::files,
                file::post_file,
                engine::engine_init,
                engine::engine_next,
            ],
        )
}
