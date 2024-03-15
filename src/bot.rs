use futures_util::StreamExt;
use core::fmt;
use std::cell::{BorrowMutError, RefCell};
use std::env;
use tracing::info;
use rive_models;
use rive_http;
use rive_autumn;
use rive_cache_inmemory;
use rive_gateway;

#[derive(Debug)]
pub struct Bot {
	http: rive_http::Client,
	autumn: rive_autumn::Client,
	cache: rive_cache_inmemory::InMemoryCache,
	gateway: RefCell<rive_gateway::Gateway>,
    pub bot_user: rive_models::user::User,
}


impl Bot {
	pub async fn new(token: String) -> Result<Bot, BotError> {
		let auth = rive_models::authentication::Authentication::BotToken(token);
		let http = rive_http::Client::new(auth.clone());
		let autumn = rive_autumn::Client::new();
		let cache = rive_cache_inmemory::InMemoryCache::new();
		let gateway = RefCell::new(rive_gateway::Gateway::connect(auth).await?);
        let bot_user = http.fetch_self().await?;
        info!("Bot init success!");	
        Ok(Bot{
			http,
			autumn,
			cache,
			gateway,
            bot_user,
		})
	}
	pub async fn next_event(&self) -> Result<rive_models::event::ServerEvent, BotError> {
		match self.gateway.try_borrow_mut()?.next().await {
            Some(res) => Ok(res?),
            None => Err(BotError::APIError)
        }
	}
    pub async fn send_message(&self, channel: String, message: String) -> Result<rive_models::message::Message, BotError> {
        info!("Sending message: \"{}\" to channel {}", message, channel);
        let data: rive_models::data::SendMessageData = rive_models::data::SendMessageData {
            content: Some(message),
            ..rive_models::data::SendMessageData::default()
        };
        Ok(self.http.send_message(channel, data).await?)
    }
}

#[derive(Debug)]
pub enum BotError {
	APIError,
	MissingToken,
    RefcellError
}
impl fmt::Display for BotError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Bot Error: {}", self.to_string())
   } 
}
impl std::error::Error for BotError {
   fn description(&self) -> &str {
       "An Error in the operation of the bot"
   } 
}
impl From<env::VarError> for BotError {
	fn from(_value: env::VarError) -> Self {
	    BotError::MissingToken
	}
}
impl From<rive_http::Error> for BotError {
	fn from(_value: rive_http::Error) -> Self {
	    BotError::APIError
	}
}
impl From<rive_gateway::Error> for BotError {
	fn from(_value: rive_gateway::Error) -> Self {
	    BotError::APIError
	}
}
impl From<BorrowMutError> for BotError {
    fn from(_value: BorrowMutError) -> Self {
        BotError::RefcellError
    }
}
