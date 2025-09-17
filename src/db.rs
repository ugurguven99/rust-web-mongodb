use mongodb::Collection;
use crate::models::Person;

pub struct AppState {
    pub persons_collection: Collection<Person>,
}
