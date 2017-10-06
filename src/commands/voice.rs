use serenity::model::Mentionable;

command!(join(ctx, msg) {
    let channel = msg.guild().and_then(|guild| {
        let mut result = None;
        for (_, vs) in guild.read().unwrap().voice_states.iter() {
            if msg.author.id == vs.user_id {
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

use serenity::voice;

command!(play(ctx, msg) {
    let guild_id = match msg.guild_id() {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say("Error finding guild id");
            return Ok(());
        }
    };

    let airhorn_sound = ".\\audio\\airhorn.dca";

    if let Some(handler) = ctx.shard.lock().manager.get(guild_id) {
        let source = match voice::dca(airhorn_sound) {
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