#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() {
    let server = zero2new::run().expect("Failed to bind address");
    // tokio::spawn returns a handle to the spawned future
    // but no use for it
    let _ = tokio::spawn(server);
}
