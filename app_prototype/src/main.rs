#[tokio::main]
async fn main() {
    let client = lib_sam_site::infrastructure::sam_site_adapter::create_client();

    lib_sam_site::infrastructure::sam_site_adapter::get_session_id(&client).await;
    lib_sam_site::infrastructure::sam_site_adapter::get_students(&client).await;
}
