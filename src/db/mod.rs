use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection(database_name: &str) -> SqliteConnection {
    SqliteConnection::establish(database_name)
        .unwrap_or_else(|_| panic!("Error connect {}",database_name))
}

pub fn post(connection :&SqliteConnection, title: &str, body: &str) {
    let post = models::NewPost {title, body};

    diesel::insert_into(schema::posts::table)
        .values(&post)
        .execute(connection)
        .expect("Error insert new task");
}

pub fn query(connection :&SqliteConnection) -> Vec<models::Post>{
    schema::posts::table
        .load::<models::Post>(connection)
        .expect("Error load tasks")

}

// pub fn filter_task(connection: &SqliteConnection) {
//     schema::task::table
//         .filter(models::Task -> )
        
// }
