use std::{env, path::Path};
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::str;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType,
};

struct Handler;

fn get_total_num_cases() -> u32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_3 col_4").descendant(Name("p"))) {
        let num_cases = element.text().parse::<u32>().unwrap();
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
    for element in document.find(Attr("class", "row_1 col_2").descendant(Name("p"))) {
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
    for element in document.find(Attr("class", "row_1 col_1").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}

fn get_employee_cases() -> i32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_2 col_4").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}

fn get_last_updated() -> std::string::String {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for when the page was last updated
    for element in document.find(Name("p")) {
        let last_updated = element.text();
        if !last_updated.contains("Updated") {
            continue;
        }

        let date = last_updated.split(")").collect::<Vec<&str>>()[0];

        // println!("{}", date);
        return date.to_string();
    }

    return "Unknown".to_string();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!cases" {
            let total_num_cases = get_total_num_cases();
            let s1: String = total_num_cases.to_string();

            let oncampus = get_oncampus_cases();
            let s2: String = oncampus.to_string();

            let offcampus = get_offcampus_cases();
            let s3: String = offcampus.to_string();

            let employees = get_employee_cases();
            let s4: String = employees.to_string();

            
            let channel_id = msg.channel_id;
            //
            // The create message builder allows you to easily create embeds and messages
            // using a builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, three fields, and a footer.
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0x790604);
                    e.thumbnail("attachment://temple.png");
                    e.title("Temple University Covid Cases");
                    // e.image("attachment://ferris_eyes.png");
                    e.fields(vec![
                        ("Total Cases  ", format!("`{}`", s1), false),
                        ("On-Campus Students  ", format!("`{}`", s2), false),
                        ("Off-Campus Students  ", format!("`{}`", s3), false),
                        ("Employees  ", format!("`{}`", s4), false),
                        // ("New Cases Since Last Update", "`0`", false),
                    ]);
                    e.footer(|f| {
                        let last_updated = get_last_updated();
                        f.text(format!("Last {})", last_updated));

                        f
                    });

                    e
                });
                m.add_file(AttachmentType::Path(Path::new("./temple.png")));
                m
            }).await;


            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
                // Probably embeds are disabled
                // Send same message without embeds
                    // Sending a message can fail, due to a network error, an
                    // authentication error, or lack of permissions to post in the
                    // channel, so log to stdout when some error happens, with a
                    // description of it.

                    let total_num_cases = get_total_num_cases();
                    let s1: String = total_num_cases.to_string();

                    let oncampus = get_oncampus_cases();
                    let s2: String = oncampus.to_string();

                    let offcampus = get_offcampus_cases();
                    let s3: String = offcampus.to_string();

                    if let Err(why) = channel_id.say(&ctx.http, format!("Total Cases: `{}`", s1)).await {
                        println!("Error sending message: {:?}", why);
                    }
                    if let Err(why) = channel_id.say(&ctx.http, format!("On-campus Cases: `{}`", s2)).await {
                        println!("Error sending message: {:?}", why);
                    }
                    if let Err(why) = channel_id.say(&ctx.http, format!("Off-campus Cases: `{}`", s3)).await {
                        println!("Error sending message: {:?}", why);
                    }
            }

        }

    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
