use std::env;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!cases" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            
            let total_num_cases = get_total_num_cases();
            let s1: String = total_num_cases.to_string();

            let offcampus = get_offcampus_cases();
            let s2: String = offcampus.to_string();

            let oncampus = get_oncampus_cases();
            let s3: String = oncampus.to_string();

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Total Cases: `{}`", s1)).await {
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("On-campus Cases: `{}`", s2)).await {
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Off-campus Cases: `{}`", s3)).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn get_total_num_cases() -> i32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_3 col_4").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}

fn get_offcampus_cases() -> i32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_3 col_2").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}
fn get_oncampus_cases() -> i32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_3 col_1").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
