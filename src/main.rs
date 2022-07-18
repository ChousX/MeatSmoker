use serenity::prelude::*;
mod handler;
use handler::Handler;

fn get_token() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(".token").expect("Can't open .token");
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Cant read .token");
    token
}

#[tokio::main]
async fn main() {
    let token = get_token();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
