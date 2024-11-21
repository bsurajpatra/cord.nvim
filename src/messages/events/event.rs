use crate::ipc::pipe::PipeServerImpl;

use super::{client::ClientEvent, local::LocalEvent, server::ServerEvent};

#[derive(Debug)]
pub enum Event {
    Client(ClientEvent),
    Local(LocalEvent),
    Server(ServerEvent),
}

impl Event {
    pub fn on_event<T: PipeServerImpl>(self, pipe: &T) {
        match self {
            Event::Client(client_event) => client_event.on_event(),
            Event::Local(local_event) => local_event.on_event(),
            Event::Server(server_event) => server_event.on_event(pipe),
        }
    }
}
