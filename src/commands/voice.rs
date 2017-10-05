use serenity::client::CACHE;
use serenity::model::Mentionable;

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
            let _ = msg.channel_id.say("Groups and DMs not supported");
            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();
    shard.manager.join(guild_id, connect_to);

    let _ = msg.channel_id.say(&format!("Joined {}", connect_to.mention()));
});

command!(leave(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            let _ = msg.channel_id.say("Groups and DMs are not supported");
            return Ok(());
        },
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