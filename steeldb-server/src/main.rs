use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use steeldb::SteelDB;
use steeldb_core::json_result::TableJSON;
use steeldb_core::{ExecutionResult, SteelDBInterface};

#[tokio::main]
async fn main() {
    let database = Arc::new(Mutex::new(SteelDB::new()));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/query", post(handle_query))
        .with_state(database);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_query(
    State(database): State<Arc<Mutex<SteelDB>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    // Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<TableJSON>) {
    // insert your application logic here
    let hello_response = TableJSON {
        table_name: "world!".to_owned(),
        columns: HashMap::new(),
        select_columns: Vec::new(),
    };
    let db_mutex = Arc::clone(&database);
    tokio::spawn(async move {
        let mut result: Option<ExecutionResult> = None;
        {
            let mut db = db_mutex.lock().unwrap();
            result = Some(db.execute("select name;".to_owned()));
        }
        return result;
    });
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(hello_response))
}
