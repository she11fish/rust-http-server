
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:8000")
        .await?
        .text()
        .await?;
    println!("{}", resp);
    Ok(())
}