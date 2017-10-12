pub mod meta;
pub mod voice;

use serenity::utils::MessageBuilder;

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
        let _ = msg.channel_id.say(&format!("Please enter two numbers"));
    } else {
        let res = first * second;

        let _ = msg.channel_id.say(&res.to_string());
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

    let _ = msg.channel_id.say(&response);
});

command!(notify(_ctx, msg, args) {
    let message = args.full();

    let response = MessageBuilder::new()
        .push("@everyone ")
        .mention(msg.author.clone())
        .push(" is playing ")
        .push_mono(message)
        .push(". Feel free to join!")
        .build();

    let _ = msg.channel_id.say(&response);
});