use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::Config;

#[derive(Debug)]
pub struct InitializeEvent {
    pub config: Config,
}

impl OnEvent for InitializeEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.logger.set_level(self.config.log_level);
        ctx.cord.config = Some(self.config);

        Ok(())
    }
}

impl InitializeEvent {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
