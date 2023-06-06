use reqwest::Client;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use warp::hyper::body::Bytes;
use warp::{Filter, Reply};

pub async fn subscribe(
    hub_url: &str,
    topic_url: &str,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    // The callback URL is where the hub will send updates.
    const CALLBACK_URL: &str = "localhost:3000/callback";

    // Send the subscription request to the hub.
    let res = client
        .post(hub_url)
        .form(&[
            ("hub.mode", "subscribe"),
            ("hub.topic", topic_url),
            ("hub.callback", CALLBACK_URL),
        ])
        .send()
        .await?;

    // Check whether the subscription was successful.
    // A successful response will have a 2xx status.
    if !res.status().is_success() {
        return Err("Subscription failed".into());
    }

    // Start the server to handle callback requests from the hub.
    let verification_route = warp::path("callback")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(|params: HashMap<String, String>| async move {
            let challenge = params.get("hub.challenge").unwrap_or(&"".to_string());
            Ok::<_, Infallible>(challenge.to_string())
        });

    let distribution_route = warp::path("callback")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(|body: Bytes| async move {
            println!("Received content: {}", String::from_utf8_lossy(&body));
            Ok::<_, Infallible>("Received content")
        });

    let routes = verification_route.or(distribution_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
