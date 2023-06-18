use better_uptime::IncidentRequest;

#[tokio::main]
async fn main() -> better_uptime::Result<()> {
    let uptime = better_uptime::Uptime {
        token: "".to_string(), // You can get this from: https://uptime.betterstack.com/team/0/teams
    };
    uptime
        .create_incident(IncidentRequest {
            requester_email: "siong@raccoons.dev".to_string(),
            name: "Test".to_string(),
            summary: "Testing only".to_string(),
            description: "Testing only".to_string(),
            ..IncidentRequest::default()
        })
        .await?;

    Ok(())
}
