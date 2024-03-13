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
	gateway: RefCell<rive_gateway::Gateway>
}

impl Bot {
	pub async fn new(token: String) -> Result<Bot, BotError> {
		let auth = rive_models::authentication::Authentication::BotToken(token);
		let http = rive_http::Client::new(auth.clone());
		let autumn = rive_autumn::Client::new();
		let cache = rive_cache_inmemory::InMemoryCache::new();
		let gateway = RefCell::new(rive_gateway::Gateway::connect(auth).await?);
        info!("Bot init success!");
		Ok(Bot{
			http,
			autumn,
			cache,
			gateway
		})
	}
	pub async fn next_event(&self) -> Result<rive_models::event::ServerEvent, BotError> {
		match self.gateway.try_borrow_mut()?.next().await {
            Some(res) => Ok(res?),
            None => Err(BotError::GatewayError)
        }
	}
}

#[derive(Debug)]
pub enum BotError {
	GatewayError,
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
impl From<rive_gateway::Error> for BotError {
	fn from(_value: rive_gateway::Error) -> Self {
	    BotError::GatewayError
	}
}
impl From<BorrowMutError> for BotError {
    fn from(_value: BorrowMutError) -> Self {
        BotError::RefcellError
    }
}
