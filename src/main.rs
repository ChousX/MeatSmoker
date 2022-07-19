use std::env;
use rand::{thread_rng, Rng};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::utils::MessageBuilder;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

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
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[group]
#[commands(help, flip, d20)]
pub struct General;


#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let subject: &str = &msg.content;
    let response = MessageBuilder::new()
    .push("Why are we here?\n")
    .push_bold_line_safe("TO SMOKE MEATS!!!")
    .push_bold_line_safe("Command List:")
    .push_bold_safe("!help")
    .push(": brings you here and lists of all the commands\n")
    .push_bold_safe("!flip")
    .push(": flips a coin\n")
    .push_bold_safe("!d20")
    .push(": rolls a d20\n")
    .build();
    msg.reply(ctx, response).await?;

    Ok(())
}

#[command]
async fn flip(ctx: &Context, msg: &Message) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_bold_safe(&msg.author.name);
    response.push(if thread_rng().gen_bool(0.5) { " Heads"} else { " Tails"});
    msg.reply(ctx, response.build()).await?;
    Ok(())
}

#[command]
async fn d20(ctx: &Context, msg: &Message) -> CommandResult{
    let result = thread_rng().gen_range(1..=20);
            let mut response = MessageBuilder::new();
            response
                .push_bold_safe(&msg.author.name)
                .push(" ")
                .push(result.to_string());
    msg.reply(ctx, response.build()).await?;
    
    Ok(())
}
