use serenity::model::UserId;
use serenity::utils::MessageBuilder;
use std::str::FromStr;

use DEFAULT_STATUS;

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

command!(setgame(ctx, msg, args) {
    let mut game = args.full();

    // Clear the game if no input is given
    if game == "" {
        game = DEFAULT_STATUS.to_string();
    }
        
    ctx.set_game_name(&game);

    let _ = msg.channel_id.say("Bot status has been updated");
});