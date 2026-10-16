use azalea::prelude::*;

#[tokio::main]
async fn main() -> AppExit {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "uhmc-iWRv.aternos.me")
        .await
}

#[derive(Clone, Component, Default)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> eyre::Result<()> {
    match event {
        // When a chat message arrives (Pull the message out and name it)
        Event::Chat(m) => {
            if let (Some(sender), content) = m.split_sender_and_content() {

                if sender == bot.username() {
                    // ignore own messages
                    return Ok(());
                }
                bot.chat(&content);

                println!("{}", content);
            }
        }


        // When bot finishes spawning into the world
        Event::Spawn => {
            bot.chat("I've Spawned!");
        }

        // Every other event: do nothing
        _ => {
        }
    }

    Ok(())
}
