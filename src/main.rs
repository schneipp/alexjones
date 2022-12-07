use serde_json::Value;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

use crate::tenor::Tenor;

mod tenor;

#[group]
#[commands(reddit,gif,badtaste,unixporn,reddit,dadjoke)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    //load .env file
    let dotenv_result = dotenv::dotenv();
    if dotenv_result.is_err() {
        println!("Could not read .env file");
    }

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    //    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}


#[command]
async fn unixporn(ctx: &Context, msg: &Message) -> CommandResult {
    println!("unixporn command");
    let search = "unixporn".to_string();
    reddit_fetcher(ctx, msg, search.as_str()).await?;
    Ok(())
}

#[command]
async fn reddit(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Reddit command");
    let search = msg.content[8..].to_string();
    reddit_fetcher(ctx, msg, search.as_str()).await?;
    Ok(())
}

#[command]
async fn badtaste(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Badtaste command");
    reddit_fetcher(ctx, msg, "antijokes").await?;
    Ok(())
}

async fn reddit_fetcher(ctx: &Context, msg: &Message, subredit: &str) -> CommandResult {
    let res = reqwest::get(format!("https://www.reddit.com/r/{}/random.json", subredit)).await?;
    let body = res.text().await?;
    println!("body: {}", body);
    let v: Value = serde_json::from_str(&body).unwrap();
    let url = v[0]["data"]["children"][0]["data"]["url"].as_str().unwrap();
//    println!("{:#?}", v);
    let title = v[0]["data"]["children"][0]["data"]["title"].clone();
    let image_preview = &v[0]["data"]["children"][0]["data"]["preview"];
    if let Some(image) = image_preview["images"][0]["source"]["url"].as_str(){
        let image = image.replace("&amp;", "&");
        let image = image.replace("amp;", "");
        let image = image.replace("preview.redd.it", "i.redd.it");
        let m = msg
            .clone()
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(title);
                    e.url(url);
                    e.image(image);
                    e
                });
                m
            })
            .await;
        if m.is_err() {
            println!("Error sending message: {:?}", m.err());
        }
    }else{
        println!("No image found");
        let selftext = v[0]["data"]["children"][0]["data"]["selftext"].as_str().unwrap();
        let m = msg
            .clone()
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(title);
                    e.url(url);
                    e.description(selftext);
                    e
                });
                m
            })
            .await;
        if m.is_err() {
            println!("Error sending message: {:?}", m.err());
        }
    }
    Ok(())
}

#[command]
async fn dadjoke(ctx: &Context, msg: &Message) -> CommandResult {
    //request with Accept header
    let res = reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?;
    //let res = reqwest::get("https://icanhazdadjoke.com/").await?;
    let body = res.text().await?;
    println!("body: {}", body);
    let v: Value = serde_json::from_str(&body).unwrap();
    let joke = v["joke"].as_str().unwrap();
    msg.reply(ctx, joke).await?;
    Ok(())
}

#[command]
async fn gif(ctx: &Context, msg: &Message) -> CommandResult {
    let tenor_token = env::var("TENOR_TOKEN").expect("tenor token");
    let gif: Tenor = reqwest::get(format!(
        "https://g.tenor.com/v1/search?q={}&key={}&limit=1",
        &msg.content[5..].to_string(),
        tenor_token
    ))
    .await?
    .json()
    .await?;
    let m = msg
        .clone()
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Gif");
                e.image(gif.results[0].media[0].gif.url.to_string());
                e
            });
            m
        })
        .await;
    if m.is_err() {
        println!("Error sending message: {:?}", m.err());
    }
    Ok(())
}
