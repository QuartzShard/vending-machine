mod bot;
use bot::Bot;
use rive_models::event::ServerEvent;
use tracing::error;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::error::Error;
use tracing::info;
use tracing_subscriber;
use rive_models::*;


// Types! I love Types! I love readable code!
// HashMap<String, HashMap<String, String>>? no thanks.
type ServerID = String;
type RoleName = String;
type EmojiID = String;
type ServerRoleMap = HashMap<ServerID, HashMap<EmojiID, RoleName>>;

// I don't know if threading will make this explode or if it's doomed to anyway
// because the bot has a RefCell in it. I'm not ever doing a join!() though, so
// no actual concurrent work happens? I think? We shall see.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
	
	tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
	info!("Starting...");

	let vendor_bot: Bot = Bot::new(env::var("TOKEN")?).await?;
	let mut role_map: ServerRoleMap = HashMap::new();
	
	// If we're a debug build, compile this bit that puts test data in the map
	#[cfg(debug_assertions)]
	{	
		info!("Debug modes populating the map with test values: ");
		let mut tmp: HashMap<EmojiID, RoleName> = HashMap::new();
		tmp.insert("1GV8ZMSVB15CYGAAJ3XKPVNXJ".to_string(), "hedgehog".to_string());
		role_map.insert("01GV8NV8QHJCYE0BGCR8SDHBN8".to_string(), tmp);
	}

	info!("Role Map:\n{:?}", role_map);		

	// Main program loop. Wait for events, respond appropriately
	loop {
        match vendor_bot.next_event().await {
            Ok(event) => match handle_event(event, &mut role_map).await {
				Ok(()) => (),
				Err(error) => error!("{}", error)
			},
            Err(error) => error!("{}", error) 
        };
    }
}

// Typing shorthand, again
type EventResult = Result<(), EventHandleError>;

// Could've been an inline, but this is prettier
async fn handle_event(event: ServerEvent, role_map: &mut ServerRoleMap) -> EventResult {
	// Log any and all event if we're a debug build
	#[cfg(debug_assertions)]
	info!("Got event: {:?}", event);

	match event {
		ServerEvent::Ready(_) => {info!("Bot Ready"); Ok(())},
		ServerEvent::Message(ev) => handle_message(ev, role_map).await,
		ServerEvent::MessageReact(ev) => handle_react(ev, role_map).await,
		ServerEvent::MessageUnreact(ev) => handle_unreact(ev, role_map).await,
		ServerEvent::MessageRemoveReaction(ev) => handle_removed_react(ev, role_map).await,
		ServerEvent::MessageDelete(ev) => handle_deleted_message(ev, role_map).await,
		ServerEvent::EmojiDelete(ev) => handle_deleted_emote(ev, role_map).await,
		ServerEvent::ServerDelete(ev) => handle_deleted_server(ev, role_map).await,		
		// If it's not one of the event listed above, we don't care. Just return Ok(())
		// as if we handled it. (Which we did, by ignoring it :trl:)
		_ => Ok(()),
	}
}

 // TODO: Actually populate handler skeletons
async fn handle_message(msg: message::Message, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}
async fn handle_react(ev: event::MessageReactEvent, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}
async fn handle_unreact(ev: event::MessageUnreactEvent, role_map: &mut ServerRoleMap) -> EventResult { 
	Ok(())
}
async fn handle_removed_react(ev: event::MessageRemoveReactionEvent, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}
async fn handle_deleted_message(ev: event::MessageDeleteEvent, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}
async fn handle_deleted_emote(ev: event::EmojiDeleteEvent, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}
async fn handle_deleted_server(ev: event::ServerDeleteEvent, role_map: &mut ServerRoleMap) -> EventResult {
	Ok(())
}

// Where did we go wrong?
#[derive(Debug)]
enum EventHandleError {
	Message,
	MessageReact,
	MessageUnReact,
	MessageRemoveReact,
	MessageDelete,
	EmojiDelete,
	ServerDelete
}
impl fmt::Display for EventHandleError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Event Error: {}", self.to_string())
   } 
}
impl std::error::Error for EventHandleError {
   fn description(&self) -> &str {
       "An Error in event handling"
   } 
}
