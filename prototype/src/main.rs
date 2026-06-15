#[tokio::main]
async fn main() {
    let client = sam_site::infrastructure::sam_site_adapter::create_client();

    sam_site::infrastructure::sam_site_adapter::get_session_id(&client).await;
    sam_site::infrastructure::sam_site_adapter::get_students(&client).await;
}
