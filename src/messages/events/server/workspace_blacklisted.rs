use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
    protocol::msgpack::{MsgPack, Serialize, ValueRef},
};

#[derive(Debug, Default)]
pub struct WorkspaceBlacklistedEvent;

impl Serialize for WorkspaceBlacklistedEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::String("workspace_blacklisted"), state)?;

        Ok(())
    }
}

impl OnEvent for WorkspaceBlacklistedEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord
            .pipe
            .write_to(ctx.client_id, &MsgPack::serialize(&self)?)?;

        Ok(())
    }
}
