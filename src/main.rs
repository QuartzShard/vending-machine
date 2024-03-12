mod bot;
use bot::{Bot,BotError};
use std::env;






#[tokio::main]
async fn main() -> Result<(), bot::BotError> {
	let vendor_bot: Bot = Bot::new(env::var("TOKEN")?).await?;
	while let Ok(event) = vendor_bot.next_event().await {
    } 
	Ok(())	
	
}
