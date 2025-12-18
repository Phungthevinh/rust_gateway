// sử dụng để gọi các thư mục khác nhau
mod app;
mod middlewares;
mod domains;

#[tokio::main]
async fn main() {
    app::lifecycle::listener().await;
}