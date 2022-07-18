use rand::prelude::*;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

const HELP: &str =
    "Why are we here?\nTO SMOKE MEATS!!!\nCommand List:\nHelp: !help\nCoin Flip: !flip";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let subject: &str = &msg.content;
        
        if subject == "!help" {
            let response = MessageBuilder::new().push(HELP).build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error Sending message: {:?}", why);
            }
        } else
        if subject == "!flip" {
            let result = if thread_rng().gen_bool(0.5) {
                " Heads".to_string()
            } else {
                " Tails".to_string()
            };

            let response = MessageBuilder::new()
                .push_bold_safe(&msg.author.name)
                .push(result)
                .build();
            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error Sending message: {:?}", why);
            }
        } else

        if subject == "!d20" {
            let result = thread_rng().gen_range(1..21);
            let response = MessageBuilder::new()
                .push_bold_safe(&msg.author.name)
                .push(result.to_string())
                .build();
            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error Sending message: {:?}", why);
            }
        }
      
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
