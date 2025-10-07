use crate::core::Core;
#[cfg(not(target_os = "android"))]
use intertrait::CastFromSync;
use std::{sync::Arc, thread::JoinHandle};

#[cfg(target_os = "android")]
pub trait CastFromSync {}

pub trait Service: CastFromSync {
    fn ident(self: Arc<Self>) -> &'static str;
    fn start(self: Arc<Self>, core: Arc<Core>) -> Vec<JoinHandle<()>>;
    fn stop(self: Arc<Self>);
}
