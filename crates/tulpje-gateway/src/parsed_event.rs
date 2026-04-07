use serde::de::DeserializeSeed as _;
use twilight_gateway::{Event, EventTypeFlags, Message};
use twilight_model::gateway::{OpCode, event::GatewayEventDeserializer};

use crate::shard_reporter::SHARD_REPORTER_EVENTS;

pub(crate) const WANTED_EVENTS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
    SHARD_REPORTER_EVENTS.bits()
        // misc opcode events, so need to decode to log actual name
        | EventTypeFlags::GATEWAY_HEARTBEAT.bits()
        | EventTypeFlags::GATEWAY_RECONNECT.bits()
        | EventTypeFlags::GATEWAY_INVALIDATE_SESSION.bits(),
);

#[derive(Debug, thiserror::Error)]
pub(crate) enum MessageParseError {
    #[error("couldn't deserialize gateway event")]
    Deserialize,
    #[error("couldn't deserialize event: {0}")]
    DeserializeEvent(serde_json::Error),
    #[error("unknown opcode: {0}")]
    UnknownOpCode(u8),
    #[error("unknown event: ({0:?}, {1:?})")]
    UnknownEvent(OpCode, Option<String>),
}

pub(crate) struct ParsedEvent {
    pub(crate) forward: bool,
    pub(crate) name: Option<String>,
    pub(crate) event: Option<Event>,
    pub(crate) text: Option<String>,
}

impl ParsedEvent {
    pub(crate) fn from_event(forward: bool, event: Event, text: Option<String>) -> Self {
        Self {
            forward,
            name: event.kind().name().map(String::from),
            event: Some(event),
            text,
        }
    }

    pub(crate) fn from_text(forward: bool, name: Option<String>, text: String) -> Self {
        Self {
            forward,
            name,
            event: None,
            text: Some(text),
        }
    }

    pub(crate) fn from_message(value: Message) -> Result<Self, MessageParseError> {
        match value {
            Message::Close(frame) => Ok(Self::from_event(false, Event::GatewayClose(frame), None)),
            Message::Text(text) => {
                let Some(deserialize) = GatewayEventDeserializer::from_json(&text) else {
                    return Err(MessageParseError::Deserialize);
                };

                let numeric_opcode = deserialize.op();
                let Some(opcode) = OpCode::from(numeric_opcode) else {
                    return Err(MessageParseError::UnknownOpCode(numeric_opcode));
                };

                let event_type = deserialize.event_type();
                let Ok(event_type_flags) = EventTypeFlags::try_from((opcode, event_type)) else {
                    return Err(MessageParseError::UnknownEvent(
                        opcode,
                        event_type.map(String::from),
                    ));
                };

                if WANTED_EVENTS.contains(event_type_flags) {
                    let mut json_deserializer = serde_json::Deserializer::from_str(&text);
                    Ok(Self::from_event(
                        opcode == OpCode::Dispatch,
                        deserialize
                            .deserialize(&mut json_deserializer)
                            .map_err(MessageParseError::DeserializeEvent)?
                            .into(),
                        Some(text),
                    ))
                } else {
                    Ok(Self::from_text(
                        opcode == OpCode::Dispatch,
                        event_type.map(String::from),
                        text,
                    ))
                }
            }
        }
    }

    pub(crate) fn is_close(&self) -> bool {
        matches!(self.event, Some(Event::GatewayClose(_)))
    }
}
