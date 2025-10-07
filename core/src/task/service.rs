use futures_util::future::BoxFuture;
#[cfg(not(target_os = "android"))]
use intertrait::CastFromSync;
use std::sync::Arc;
use thiserror::Error;

#[cfg(target_os = "android")]
pub trait CastFromSync {}

#[derive(Error, Debug)]
pub enum AsyncServiceError {
	#[error("{0}")]
	Service(String),
}

pub type AsyncServiceResult<T> = std::result::Result<T, AsyncServiceError>;

pub type AsyncServiceFuture = BoxFuture<'static, AsyncServiceResult<()>>;

pub trait AsyncService: CastFromSync {
	fn ident(self: Arc<Self>) -> &'static str;
	fn start(self: Arc<Self>) -> AsyncServiceFuture;
	fn signal_exit(self: Arc<Self>);
	fn stop(self: Arc<Self>) -> AsyncServiceFuture;
}
