use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // first step set up a client
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);
    //println!("client : {:?}", client)
    let resp = client.list_buckets().send().await?;

    let buckets = resp.buckets();
    let num_buckets = buckets.len();
    // list s3 bucket
    println!("Num of buckets {:?}", num_buckets);
    Ok(())
}
