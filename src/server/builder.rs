use std::time::Instant;

use crate::{config::Config, server::{Server, ServerData}};

pub struct ServerBuilder {
    cfg: Config
}

impl ServerBuilder {
    pub fn new(cfg: Config) -> ServerBuilder{
        ServerBuilder { cfg }
    }

    pub fn build(&self) -> Server{
        Server {
            cfg: self.cfg.clone(),
            data: ServerData {
                start_time: Instant::now()
            }
        }
    }
}