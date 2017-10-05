#[macro_use] 
extern crate serenity;

mod commands;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{DispatchError, StandardFramework, help_commands};
use serenity::utils::MessageBuilder;
use serenity::Result as SerenityResult;
use serenity::voice;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c 
            .allow_whitespace(true)
            .on_mention(true)
            .prefix("~")
            .delimiters(vec![", ", ","]))
        .before(|_ctx, msg, command_name| {
            println!("Got command '{}' by user '{}' ", command_name, msg.author.name);
            true // If `before` returns false, command processing doesn't happen
        })
        .after(|_, _, command_name, error| {
            match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        .on_dispatch_error(|_ctx, msg, error| {
            if let DispatchError::RateLimited(seconds) = error {
                let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
            }
        })
        // Can't be used more that once per 5 seconds
        .simple_bucket("emoji", 5)
        // Can't be used more than 2 times per 30 seconds, with a 5 second delay
        .bucket("complicated", 5, 30, 2) 
        .group("Meta", |g| g
            .command("about", |c| c.exec(commands::meta::about))
            .command("ping", |c| c.exec(commands::meta::ping))
            .command("help", |c| c.exec_help(help_commands::plain)))
        .group("Emoji", |g| g 
            .prefix("emoji")
            .command("cat", |c| c 
                .exec_str(":cat:")
                .desc("Sends an emoji with a cat.")
                .bucket("emoji"))
            .command("dog", |c| c 
                .exec_str(":dog:")
                .desc("Sends an emoji with a dog.")
                .bucket("emoji")))
        .group("Voice", |g| g 
            .command("join", |c| c.exec(commands::voice::join))
            .command("leave", |c| c.exec(commands::voice::leave)))
        .command("multiply", |c| c 
            .exec(multiply)
            .known_as("*")
            .num_args(2)
            .desc("multiplies two numbers")
            .example("1.3 4"))
        .command("love", |c| c.exec(love)),
    );

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

command!(multiply(_ctx, msg, args) {
    let mut valid = true;
    // Parse both arguments at the same time. 
    // If we don't get back results from both arguments then we don't care what either one is. 
    let (first, second) = match (args.single::<f64>(), args.single::<f64>()) {
        (Ok(x), Ok(y))=> (x, y),
        (_, _) => {
            valid = false;
            (-1.0, -1.0)
        },
    };

    if !valid {
        check_msg(msg.channel_id.say(&format!("Please enter two numbers")));
    } else {
        let res = first * second;

        check_msg(msg.channel_id.say(&res.to_string()));
    }
});

command!(love(_ctx, msg, _args) {
    let mut target = msg.author.clone();

    if msg.mentions.len() >= 1 {
        target = msg.mentions[0].clone();
    }

    let response = MessageBuilder::new()
        .push("I love you ")
        .mention(target)
        .push("!")
        .build();

    check_msg(msg.channel_id.say(&response));
});

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}