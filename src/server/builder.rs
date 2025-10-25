use crate::{config::Config, server::Server};

pub struct ServerBuilder {
    cfg: Config
}

impl ServerBuilder {
    pub fn new(cfg: Config) -> ServerBuilder{
        ServerBuilder { cfg }
    }

    pub fn build(&self) -> Server{
        Server {
            cfg: self.cfg.clone()
        }
    }
}