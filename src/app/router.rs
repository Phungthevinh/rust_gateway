use axum::{
    routing::get,
    Router,
};


pub fn build_router() -> Router<()>{
    println!("đang chạy");
    Router::new()
        .route("/", get(|| async {"Hello"}))
}