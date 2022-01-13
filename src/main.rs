use aws_sdk_s3::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let req = client.list_buckets();
    
    let resp = req.send().await;
    println!("buckets: {:?}", resp);
    Ok(())
}
