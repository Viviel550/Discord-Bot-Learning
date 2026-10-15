use std::env;
use serenity::{all::EventHandler, async_trait, model::channel::Message, prelude};

struct Handler;
/*
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Test").await {
                println!("JOEVR {why}");
            }
        }
    }
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename(".env.production")?;
    let token = dotenvy::var("DISCORD_TOKEN")?;
    println!("{}", token);
    Ok(())
}
