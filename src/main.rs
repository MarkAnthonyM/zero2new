use zero2new::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
