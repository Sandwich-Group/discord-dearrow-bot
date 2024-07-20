mod dearrow;

use regex_lite::Regex;
use serenity::{
    all::{CreateAllowedMentions, CreateEmbed, CreateMessage, GatewayIntents, Message},
    async_trait,
    prelude::*,
};

const YOUTUBE_REGEX: &str = r"https?://(?:www\.)?(?:m\.)?(?:youtube\.com/watch\?v=|youtube\.com/(?:v|embed|live)/|youtu\.be/)([a-zA-Z0-9-_]{11})";

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            async_main().await;
        });
}

async fn async_main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler::new())
        .intents(intents)
        .await
        .expect("Err creating client");

    println!("Starting client");
    client.start().await.expect("Err starting client");
}

struct Handler {
    youtube_regex: Regex,
}

impl Handler {
    fn new() -> Self {
        Self {
            youtube_regex: Regex::new(YOUTUBE_REGEX).unwrap(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore bot or system messages to prevent false positives and infinite loops.
        if msg.author.bot || msg.author.system {
            return;
        }

        // Use `is_match` to check if message contains a youtube link. We don't immediately use `captures` because this
        // is a rare case.
        if self.youtube_regex.is_match(&msg.content) {
            let group = self.youtube_regex.captures(&msg.content).unwrap();
            let code = group.get(1).unwrap().as_str();
            println!("Detected YouTube link: {code}");

            let branding = match dearrow::fetch_new_title(code).await {
                Ok(Some(x)) => x,
                Ok(None) => {
                    println!("Video {code} has no custom DeArrow title");
                    return;
                }
                Err(e) => {
                    println!("Error fetching branding for {code}: {e:?}");
                    return;
                }
            };

            println!("Detected DeArrow title for {code}: {branding}");

            let message = CreateMessage::new()
                .embed(CreateEmbed::new().title(branding))
                .allowed_mentions(CreateAllowedMentions::new())
                .reference_message(&msg);

            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
