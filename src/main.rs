// sử dụng để gọi các thư mục khác nhau
mod app;
#[tokio::main]
async fn main() {
    app::lifecycle::listener().await;
}