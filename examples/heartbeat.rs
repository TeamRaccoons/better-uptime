#[tokio::main]
async fn main() -> better_uptime::Result<()> {
    // https://uptime.betterstack.com/api/v1/heartbeat/XXX, the XXX part is the identifier
    let identifier = "XXX".to_string();
    let uptime = better_uptime::Uptime {
        token: "".to_string(), // We can ignore this since heartbeat doesn't use it
    };
    uptime.heartbeat(identifier).await?;

    Ok(())
}
