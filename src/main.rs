use std::{collections::HashSet, env};

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut ultimora_cringe = HashSet::new();
    ultimora_cringe.insert(-1001067340713); // https://t.me/ultimora
    ultimora_cringe.insert(-1001398867876); // https://t.me/ultimoralive
    ultimora_cringe.insert(-1001056710815); // https://t.me/ultimora24
    ultimora_cringe.insert(-1001485363608); // https://t.me/ultimorapolitics
    ultimora_cringe.insert(-1001142744746); // https://t.me/ultimorafocus

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        match update {
            Ok(update) => {
                if let UpdateKind::Message(message) = &update.kind {
                    if let Some(forward) = &message.forward {
                        if let ForwardFrom::Channel {
                            channel,
                            message_id: _,
                        } = &forward.from
                        {
                            if ultimora_cringe.contains(&(channel.id.into())) {
                                api.send(message.text_reply(format!(
                                    "Ultim'ora detectato cringe eliminato, non provarci mai più {}",
                                    message.from.first_name
                                )))
                                .await?;
                                let deleted = api.send(message.delete()).await;
                                if deleted.is_err() {
                                    api.send(message.text_reply(
                                        "Volevo blastare questo cringe ma non sono abbastanza potente",
                                    )).await?;
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    Ok(())
}
