use mini_redis::{client, Result};

use tokio::task;

#[tokio::main]
pub async fn main() -> Result<()> {
    // let mut client = client::connect("localhost:6379").await?;
    // client.set("hello", "world".into()).await?;
    //
    // let result = client.get("hello").await?;
    // println!("got value from the server; result={:?}", result);
    //
    // Ok(())

    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });

    Ok(())
}