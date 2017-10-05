use serenity::model::UserId;
use serenity::utils::MessageBuilder;
use std::str::FromStr;

command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(about(_ctx, msg) {
    let about_response = MessageBuilder::new()
        .push("A bot created by ")
        .mention(UserId::from_str("<@107317058450042880>").unwrap())
        .push("\nSource code available at <https://github.com/bagelhobbit/ShireBot>")
        .build();

    let _ = msg.channel_id.say(about_response);
});