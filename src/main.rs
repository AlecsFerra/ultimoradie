use std::{collections::HashSet, env, fs};

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    println!("Starting");

    let ultimora_cringe: HashSet<i64> = fs::read_to_string("ultimora_cringe.txt")
                                            .expect("ultimora_cringe.txt not found")
                                            .lines()
                                            .filter(|x| !x.is_empty())
                                            .filter(|x| !x.starts_with("#"))
                                            .map(|x| x.trim().parse().expect("One of the id is malformed"))
                                            .collect();

    println!("Loaded {} ids:", ultimora_cringe.len());
    for &id in &ultimora_cringe {
        println!("{}", id);
    }


    println!("Started polling");
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
