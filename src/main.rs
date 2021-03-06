#[macro_use] 
extern crate serenity;

mod commands;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{DispatchError, StandardFramework, help_commands};
use std::env;

struct Handler;

const DEFAULT_STATUS: &'static str = "~help";

impl EventHandler for Handler {
    fn on_ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_game_name(DEFAULT_STATUS);
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
        .simple_bucket("voice", 5) 
        .group("Meta", |g| g
            .command("about", |c| c.exec(commands::meta::about))
            .command("ping", |c| c.exec(commands::meta::ping))
            .command("help", |c| c.exec_help(help_commands::plain))
            .command("setgame", |c| c
                .exec(commands::meta::setgame)
                .required_permissions(Permissions::ADMINISTRATOR)))
        .group("Emoji", |g| g 
            .command("cat", |c| c 
                .exec_str(":cat:")
                .desc("Sends a cat emoji."))
            .command("dog", |c| c 
                .exec_str(":dog:")
                .desc("Sends a dog emoji.")))
        .group("Voice", |g| g 
            .command("join", |c| c
                .exec(commands::voice::join)
                .desc("Bot will join the user's current voice channel."))
            .command("leave", |c| c.exec(commands::voice::leave))
            .command("play", |c| c
                .exec(commands::voice::play)
                .bucket("voice"))
            .command("airhorn", |c| c
                .exec(commands::voice::airhorn)
                .bucket("voice"))
            .command("patrick", |c| c
                .exec(commands::voice::patrick)
                .bucket("voice")))
        .command("multiply", |c| c 
            .exec(commands::multiply)
            .known_as("*")
            .num_args(2)
            .desc("Multiplies two numbers.")
            .example("1.3 4"))
        .command("love", |c| c
            .exec(commands::love)
            .desc("Sends a message to you or a friend.")
            .usage("<@friend>"))
        .command("playing", |c| c
            .exec(commands::notify)
            .desc("Notify everyone what you're playing and invite them to join.")),
    );

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}