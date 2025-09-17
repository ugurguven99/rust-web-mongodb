use std::i32;
use actix_web::{delete, web, HttpResponse, Responder};
use crate::models::Person;
use crate::db::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::stream::StreamExt;
use serde_json::json;
use mongodb::{Collection, bson::doc};
use mongodb::bson::oid::ObjectId;
use crate::models::UpdatePerson;
use crate::models::NewPerson;
use mongodb::options::FindOneOptions;

pub async fn get_persons(state: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = state.lock().await;
    let collection = &state.persons_collection;

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut persons = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(person) => persons.push(person),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Error fetching document: {}", e)),
        }
    }

    HttpResponse::Ok().json(persons)
}
pub async fn delete_person(data: web::Data<Arc<Mutex<AppState>>>, id_path: web::Path<i32>) -> HttpResponse {
    let id = id_path.into_inner();
    let state = data.lock().await;
    let collection = &state.persons_collection;

    match collection.delete_one(doc! { "person_id": id }, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn update_person(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<i32>,
    person_data: web::Json<UpdatePerson>,
) -> HttpResponse {
    let id = path.into_inner();

    let update = doc! {
        "$set": {
            "isim": &person_data.isim,
            "soyisim": &person_data.soyisim,
            "email": &person_data.email,
            "yas": &person_data.yas,
        }
    };
    let state = data.lock().await;
    let collection = &state.persons_collection;

    match collection.update_one(doc! { "person_id": id }, update, None).await {
        Ok(result) => {
            if result.matched_count > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_person(
    data: web::Data<Arc<Mutex<AppState>>>,
    new_person: web::Json<NewPerson>,
) -> HttpResponse {


    let state = data.lock().await;
    let collection = &state.persons_collection;

    // ID'nin otomatik artışını kontrol et
    let mut highest_id = 0;

    // Cursor'ı bir stream olarak kullanma
    let mut cursor = collection.find(None, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(person) => {
                if person.id > highest_id {
                    highest_id = person.id;
                }
            },
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    }

    let new_id = highest_id + 1;

    let person = Person {
        id: new_id,
        isim: new_person.isim.clone(),
        soyisim: new_person.soyisim.clone(),
        email: new_person.email.clone(),
        yas: new_person.yas,
    };

    match collection.insert_one(person, None).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("Error inserting person: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub async fn get_next_person_id(
    collection: &mongodb::Collection<Person>
) -> i32 {
    let find_options = FindOneOptions::builder()
        .sort(doc! { "person_id": -1 }) // En büyük id'ye göre sıralama
        .build();

    match collection.find_one(None, find_options).await {
        Ok(Some(person)) => person.id + 1,
        Ok(None) => 1, // Eğer koleksiyon boşsa ilk id olarak 1 döner
        Err(_) => panic!("En büyük id bulunamadı"),
    }
}


