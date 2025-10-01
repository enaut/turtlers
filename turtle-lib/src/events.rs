use bevy::prelude::{Entity, Message};

#[derive(Message)]
pub struct DrawingStartedEvent(pub Entity);
