use actix_files::Files;
use actix_web::App;

fn main() {
    let _ = App::new().service(Files::new("/server", "/"));
}
