/* 
 * This bot has a single command, ping, which pings a role with the name "Pingable" in the Discord server. The command checks if the message was sent in a guild (i.e. a server), and if so, it attempts to find the role with the given name. If the role is found, the bot sends a message mentioning the role in the channel where the command was used.
 * To use this bot, you will need to replace YOUR_BOT_TOKEN with your own Discord bot token, and make sure that the role with the name "Pingable" exists in the server where you want to use the command. You will also need to add the bot to your server and give it the necessary permissions to send messages and mention roles.
 * This is just a basic example, and you can modify and extend the bot to add more functionality as needed. For more information, you can refer to the official Discord API documentation and the Serenity library documentation.
 * 
 */

use serenity::{
    client::Client,
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::channel::Message,
};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild() {
        Some(guild) => guild,
        None => return Ok(()),
    };

    let role_name = "Pingable";
    let role = match guild.role_by_name(role_name) {
        Some(role) => role,
        None => return Ok(()),
    };

    let message = format!("{} was pinged!", role.mention());
    msg.channel_id.say(&ctx.http, message)?;

    Ok(())
}

fn main() {
    let token = "YOUR_BOT_TOKEN";
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
