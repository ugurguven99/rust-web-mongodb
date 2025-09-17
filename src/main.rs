mod models;
mod db;
mod handlers;

use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware::Logger, Responder};
use mongodb::Client;
use mongodb::Collection;
use models::Person;
use handlers::{get_persons, delete_person, update_person, add_person};
use db::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn read_page() -> impl Responder {
    fs::NamedFile::open_async("./public/list.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // MongoDB Client'ı oluşturun ve veritabanına bağlanın
    let client = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
    let database = client.database("Yeni");
    let persons_collection: Collection<Person> = database.collection("Person");

    // AppState'i oluşturun ve Arc<Mutex<>> ile sarın
    let app_state = Arc::new(Mutex::new(AppState {
        persons_collection,
    }));

    // Actix-web sunucusunu başlatın
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone())) // Uygulama verisini paylaşmak için kullanılır
            .route("/read", web::get().to(read_page)) // HTML sayfasını servise sunar
            .route("/persons", web::get().to(get_persons)) // Tüm kişileri getirir
            .route("/delete_person/{id}", web::delete().to(delete_person)) // Belirli bir kişiyi siler
            .route("/update_person/{id}", web::put().to(update_person)) // Belirli bir kişiyi günceller
            .route("/add_person", web::post().to(add_person)) // Yeni bir kişi ekler
            .service(fs::Files::new("/", "./public").index_file("insert.html")) // Statik dosyaları servis eder
    })
        .bind("127.0.0.1:8080")? // Sunucuyu belirtilen adreste ve portta bağlar
        .run()
        .await
}
