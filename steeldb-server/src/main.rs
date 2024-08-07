use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::{Arc, Mutex};
use steeldb::SteelDB;
use steeldb_core::json_result::{TableJSON, QueryResultJSON, UserQueryJSON};
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
    Json(payload): Json<UserQueryJSON>,
) -> (StatusCode, Json<QueryResultJSON>) {
    // insert your application logic here
    let db_mutex = Arc::clone(&database);
    let result: ExecutionResult;
    {
        let mut db = db_mutex.lock().unwrap();
        result = db.execute(payload.user_query);
    }
    match result {
        ExecutionResult::TableResult(table) => {
            // TODO: avoid cloning data here
            let result = QueryResultJSON {
                table_result: Some(TableJSON {
                    table_name: table.get_table_name().clone(),
                    columns: table.get_columns().clone(),
                    select_columns: table.get_select_columns().clone(),
                }),
                message: "query successful".to_string(),
                status_code: StatusCode::OK.as_u16()
            };
            return (
                StatusCode::OK,
                Json(result),
            );
        }
        ExecutionResult::ParseError(error) => {
            return (StatusCode::BAD_REQUEST, Json(QueryResultJSON{
                table_result: None,
                message: format!("failed to execute query: {error}"),
                status_code: StatusCode::BAD_REQUEST.as_u16()
            }));
        }
        ExecutionResult::CommandError(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(QueryResultJSON{
                table_result: None,
                message: format!("failed to execute query: {error}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
            }));
        }
        ExecutionResult::VoidOK => {
            return (StatusCode::OK, Json(
                QueryResultJSON {
                    table_result: None,
                    message: format!("Query successful"),
                    status_code: StatusCode::OK.as_u16()
                }
            ))
        }
    }
}
