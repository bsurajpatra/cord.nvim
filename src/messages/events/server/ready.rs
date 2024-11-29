use std::sync::atomic::Ordering;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::EventContext;
use crate::protocol::msgpack::{MsgPack, serialize::Serialize, value::ValueRef};
use crate::messages::events::event::OnEvent;

#[derive(Debug, Default)]
pub struct ReadyEvent;

impl OnEvent for ReadyEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if !ctx.cord.rich_client.is_ready.swap(true, Ordering::SeqCst) {
            ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;
        }

        Ok(())
    }
}

impl Serialize for ReadyEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::String("ready"), state)?;

        Ok(())
    }
}
