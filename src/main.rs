#[macro_use] 
extern crate serenity;

use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{Args, Command, DispatchError, StandardFramework, help_commands};
use serenity::utils::MessageBuilder;
use serenity::Result as SerenityResult;
use serenity::voice;
use serenity::client::CACHE;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler);

    let about_response = MessageBuilder::new()
        .push("A test bot created by ")
        // This is my UserId so this shouldn't fail
        .mention(UserId::from_str("<@107317058450042880>").unwrap())
        .push("\nSource code available at https://github.com/bagelhobbit/ShireBot")
        .build();

    client.with_framework(
        // Configure the client, allowing for options to mutate how the
        // framework functions. 
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
        // Set a function that's called whenever a command's execution didn't complete for some reason. 
        .on_dispatch_error(|_ctx, msg, error| {
            if let DispatchError::RateLimited(seconds) = error {
                let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
            }
        })
        // Can't be used more that once per 5 seconds
        .simple_bucket("emoji", 5)
        // Can't be used more than 2 times per 30 seconds, with a 5 second delay
        .bucket("complicated", 5, 30, 2) 
        .command("about", |c| c.exec_str(&about_response))
        .command("help", |c| c.exec_help(help_commands::plain))
        .group("Emoji", |g| g 
            .prefix("emoji")
            .command("cat", |c| c 
                .desc("Sends an emoji with a cat.")
                .batch_known_as(vec!["kitty", "neko"]) // Adds multiple aliases
                .bucket("emoji") // Use the "emoji" bucket
                .exec_str(":cat:"))
            .command("dog", |c| c 
                .desc("Sends an emoji with a dog.")
                .bucket("emoji")
                .exec_str(":dog:")))
        .command("multiply", |c| c 
            .known_as("*") //Let's us call ~* instead of ~multiply
            .num_args(2)
            .desc("multiplies two numbers")
            .usage("x y")
            .example("1.3 4.5")
            .exec(multiply))
        .command("ping", |c| c 
            .check(owner_check)
            .exec_str("Pong!"))
        .command("role", |c| c 
            .exec(about_role)
            //Limits the usage of this command to the roles named
            .allowed_roles(vec!["IT", "Admin"]))
        .command("love", |c| c 
            .exec(love))
        .on("join", join)
        .on("leave", leave),
    );

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

// A function which acts as a "check", to determine whether to call a command
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &Arc<Command>) -> bool {
    msg.author.id == 107317058450042880
}

command!(about_role(_ctx, msg, args) {
    let potential_role_name = args.full();

    if let Some(guild) = msg.guild() {
        // `role_by_name` allows us to attempt attaining a reference to a role via its name
        if let Some(role) = guild.read().unwrap().role_by_name(&potential_role_name) {
            check_msg(msg.channel_id.say(&format!("Role-ID: {}", role.id)));
            return Ok(());
        }
    }

    check_msg(msg.channel_id.say(&format!("Could not find role named: {:?}", potential_role_name)));
});

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

command!(join(ctx, msg) {

    let mut channel = None;

    if let Some(guild) = msg.guild() {
        for (_, vs) in guild.read().unwrap().voice_states.iter() {
            if msg.author.id == vs.user_id {
                channel = vs.channel_id;
            }
        }
    };

    let connect_to = match channel {
        Some(id) => id,
        None => {
            return Ok(()); 
        }
    };

    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));
            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();
    shard.manager.join(guild_id, connect_to);

    check_msg(msg.channel_id.say(&format!("Joined {}", connect_to.mention())));
});

command!(leave(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs are not supported"));
            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();
    let has_handler = shard.manager.get(guild_id).is_some();

    if has_handler {
        shard.manager.remove(guild_id);
        check_msg(msg.channel_id.say("Left voice channel"));
    } else {
        check_msg(msg.channel_id.say("Not in a voice channel"));
    }
});

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}