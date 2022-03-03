use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // [§03-async/awaitを使う]この時点ではsay_worldの実行は行われない
    let op = say_world();
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("foo", "bar".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);
    // [§03-async/awaitを使う]このタイミングでようやくsay_worldの実行が行われる
    op.await;

    Ok(())
}

async fn say_world() {
    println!("Hello, world!");
}
