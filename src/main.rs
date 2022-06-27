use serenity::{
    async_trait,
    http::CacheHttp,
    model::{
        channel::{GuildChannel, Message},
        gateway::Ready,
        guild::Member,
        id::{ChannelId, UserId},
    },
    client::Context,
    prelude::*,
    Client, FutureExt,
};

const HELP_MESSAGE: &str = "The bot is saying: \"Hello world\"";

const HELLO_CMD: &str = "!hello";
const DIRTYTALK_CMD: &str = "!dirty";

#[derive(Debug)]
struct Handler;

#[async_trait]

impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}", msg.content);
        let cmd = msg.content.split(' ').collect::<Vec<&str>>();
        match cmd[0] {
            HELLO_CMD => send_hello(&ctx, &msg).await,
            DIRTYTALK_CMD => {
                dirtytalk(&ctx, &msg, &cmd[1..]).await;
            }
            _ => (),
        }

        println!("{:?}, {}", msg, cmd[0]);
    }

    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        // new_member.user.create_dm_channel(cache_http)
    }

    async fn channel_create(&self, _ctx: Context, _channel: &GuildChannel) {
        _channel.say(&_ctx.http, "content").await.expect("ded");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn send_hello(ctx: &Context, msg: &Message) {
    let author_name = &msg.author.name;

    let content = format!("Hello cái địt mẹ mày {}", author_name);
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        panic!("Ded");
    }
}

async fn dirtytalk(ctx: &Context, msg: &Message, args: &[&str]) {
    let author_name = &msg.author.name;

    let content = format!("Hello cái địt mẹ mày {}", author_name);
    let userid = UserId(383470608224223232);
    let pserver = userid.create_dm_channel(&ctx.http()).await.expect("ded");
    pserver.say(&ctx.http, "test dirty").await.expect("ded");
    msg.channel_id.say(&ctx.http, content).await.expect("ded");
    // if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
    //     panic!("Ded");
    // }
}
#[tokio::main]
async fn main() {
    let token = "OTkwNTI0Mzk4Mzc5MzU2MTgx.GryLcJ.eJHVmY9gDrWKzOiZb1Dt4ia_4x28RSCQVElKlI";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
