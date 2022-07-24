use std::env;

use rand::{thread_rng, Rng};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::{Message, AttachmentType};
use serenity::utils::MessageBuilder;
mod assets;
pub struct Handler;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::prelude::*;
use tokio::sync::RwLock;

struct FBI_Helper;

impl TypeMapKey for FBI_Helper {
    type Value = Arc<RwLock<assets::boobies::BoobieIndexer>>;
}

struct BoodyHelper;

impl TypeMapKey for BoodyHelper {
    type Value = Arc<RwLock<assets::boodies::BootieIndexer>>;
}


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

    {
        let mut data = client.data.write().await;
        data.insert::<FBI_Helper>(Arc::new(RwLock::new(assets::boobies::BoobieIndexer::start_up(BOOBY))));
        data.insert::<BoodyHelper>(Arc::new(RwLock::new(assets::boodies::BootieIndexer::start_up(BOODY))));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
#[group]
#[commands(help, flip, d20, perv, showmeboobies, fbicertify, showmeboody, fbbcertify)]
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
    .push_bold_safe("!perv")
    .push(": shows you a list of nsfw commands please be good\n     note: if abused it will be removed\n")
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





const BOODY: &str = ".boodies";
const BOOBY: &str = "./.booby";


#[command]
async fn showmeboody(ctx: &Context, msg: &Message) -> CommandResult{
    //if msg.author.has_role(cache_http, guild, role) { return Ok(())}
    
    let  boodie_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<BoodyHelper>().expect("Expected MessageCount in TypeMap.").clone()
    };

    let boodies = boodie_lock.read().await;
    if boodies.len() == 0 { return Ok(())}
    let (image, name) = boodies.get_random_image();
    if let Some(url) = image{
        msg
        .channel_id
        .send_message(&ctx.http, |m| {
            
    
            // Ping the replied user
            m.allowed_mentions(|am| {
                am.replied_user(true);
                am
            });
    
    
            // Attach image
            m.add_file(AttachmentType::Image(url));
            m
            
        }).await;
        if name != ""{
            let mut response = MessageBuilder::new();
            response.push_bold_safe(name);
            msg.reply(ctx, response.build()).await?;
        }
    }
    Ok(())
}

#[command]
async fn fbbcertify(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let  boodies_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<BoodyHelper>().expect("Expected MessageCount in TypeMap.").clone()
    };
    {
        let url = match args.single_quoted::<String>() {
            Ok(x) => x,
            Err(_) => {
                return Ok(());
            },
        };
        let mut  boodie_base= boodies_lock.write().await;

        boodie_base.add_boody(BOODY, &url, None, Some(msg.author.name.clone()));
        Ok(())
    }
}


#[command]
async fn fbicertify(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let  boobies_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<FBI_Helper>().expect("Expected MessageCount in TypeMap.").clone()
    };
    {
        let url = match args.single_quoted::<String>() {
            Ok(x) => x,
            Err(_) => {
                return Ok(());
            },
        };
        let mut  boobie_base= boobies_lock.write().await;

        boobie_base.add_booby(BOOBY, &url, None, Some(msg.author.name.clone()));
        Ok(())
    }
}

#[command]
async fn showmeboobies(ctx: &Context, msg: &Message) -> CommandResult{
    //if msg.author.has_role(cache_http, guild, role) { return Ok(())}
    
    let  boobies_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<FBI_Helper>().expect("Expected MessageCount in TypeMap.").clone()
    };

    let boobies = boobies_lock.read().await;
    if boobies.len() == 0 { return Ok(())}
    let (image, name) = boobies.get_random_image();
    if let Some(url) = image{
        msg
        .channel_id
        .send_message(&ctx.http, |m| {
            
    
            // Ping the replied user
            m.allowed_mentions(|am| {
                am.replied_user(true);
                am
            });
    
    
            // Attach image
            m.add_file(AttachmentType::Image(url));
            m
            
        }).await;
        if name != ""{
            let mut response = MessageBuilder::new();
            response.push_bold_safe(name);
            msg.reply(ctx, response.build()).await?;
        }
    }
    Ok(())
}

#[command]
async fn perv(ctx: &Context, msg: &Message) -> CommandResult{
    let subject: &str = &msg.content;
    let response = MessageBuilder::new()
    //start
    .push("This is a list of nsfw commands\n")
    .push_bold_line_safe("Please be respectfull!")
    //command list
    .push_bold_line_safe("Command List:")
    .push_bold_safe("!perv")
    .push(": brings you here and lists of all the commands\n")
    //boobies
    .push_bold_safe("!showmeboobies")
    .push(": shows you a pecture of breasts\n")
    .push_bold_safe("!fbicertify")
    .push(": will add a picture to breasts index (!fbicertify IMAGE-URL)\n")
    .push("     note: submitters are recored with the image url\n")
    //boodies
    .push_bold_safe("!showmeboody")
    .push(": shows you a pecture of butt\n")
    .push_bold_safe("!fbbcertify")
    .push(": will add a picture to boody index (!fbbcertify IMAGE-URL)\n")
    .push("     note: submitters are recored with the image url\n")
    .build();
    msg.reply(ctx, response).await?;
    Ok(())
}