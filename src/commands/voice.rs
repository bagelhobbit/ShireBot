use serenity::prelude::*;
use serenity::framework::standard::Args;
use serenity::voice;
use std::collections::HashMap;

command!(join(ctx, msg) {
    let mut target = msg.author.id;

    if msg.mentions.len() >= 1 {
        target = msg.mentions[0].id;
    }

    let channel = msg.guild().and_then(|guild| {
        let mut result = None;
        for (_, vs) in guild.read().unwrap().voice_states.iter() {
            if target == vs.user_id {
                result = vs.channel_id;
            }
        }
        result
    });

    let connect_to = match channel {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say("You need to join a voice channel first");
            return Ok(()); 
        }
    };

    let guild_id = match msg.guild_id() {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say("Groups and DMs not supported");
            return Ok(());
        }
    };

    let mut shard = ctx.shard.lock();
    shard.manager.join(guild_id, connect_to);

    let _ = msg.channel_id.say(&format!("Joined {}", connect_to.mention()));
});

command!(leave(ctx, msg) {
    let guild_id = match msg.guild_id() {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say("Groups and DMs not supported");
            return Ok(());
        }
    };

    let mut shard = ctx.shard.lock();
    let has_handler = shard.manager.get(guild_id).is_some();

    if has_handler {
        shard.manager.remove(guild_id);
        let _ = msg.channel_id.say("Left voice channel");
    } else {
        let _ = msg.channel_id.say("Not in a voice channel");
    }
});

command!(play(ctx, msg, args) {
    let sounds: HashMap<&str, &str> = 
        [("airhorn", ".\\audio\\airhorn.dca"),
         ("patrick", ".\\audio\\patrick.dca")]
         .iter().cloned().collect();

    let guild_id = match msg.guild_id() {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say("Error finding guild id");
            return Ok(());
        }
    };

    let sound = match args.get(0) {
        Some(sound) => sound,
        None => "airhorn",
    };

    let path = match sounds.get(sound) {
        Some(path) => path,
        None => {
            let _ = msg.channel_id.say("Couldn't find file");
            return Ok(());
        },
    };


    if let Some(handler) = ctx.shard.lock().manager.get(guild_id) {
        let source = match voice::dca(path) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                let _ = msg.channel_id.say("Couldn't find file");

                return Ok(());
            },
        };

        handler.play(source);

        let _ = msg.channel_id.say("Playing");
    } else {
        let _ = msg.channel_id.say("Not in a voice channel");
    }
});

command!(airhorn(ctx, msg, _args) {
    let _ = play(ctx, msg, Args::new("airhorn", ","));
});

command!(patrick(ctx, msg, _args) {
    let _ = play(ctx, msg, Args::new("patrick", ","));
});