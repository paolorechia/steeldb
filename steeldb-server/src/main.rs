use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    let result: ExecutionResult;
    {
        let mut db = db_mutex.lock().unwrap();
        result = db.execute("select name;".to_owned());
    }
    match result {
        ExecutionResult::TableResult(table) => {
            // TODO: avoid cloning data here
            return (
                StatusCode::CREATED,
                Json(TableJSON {
                    table_name: table.get_table_name().clone(),
                    columns: table.get_columns().clone(),
                    select_columns: table.get_select_columns().clone(),
                }),
            );
        }
        // TODO: decide how to handle other cases that don't include a table
        // One option:
        // https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs

        // Another option (probably easier): always include an empty table
        _ => {
            return (StatusCode::CREATED, Json(hello_response));
        }
    }
}
