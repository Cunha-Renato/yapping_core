use core::time;
use std::{collections::HashMap, time::{Duration, Instant}};
use l3gion_rust::UUID;
use super::{Response, ServerMessage, ServerMessageContent};

#[derive(Debug)]
struct TimeoutMessage {
    arrival: Instant,
    content: ServerMessageContent,
}

#[derive(Debug, Default)]
pub struct ComsManager {
    received_waiting: HashMap<UUID, ServerMessageContent>,
    sent_waiting: HashMap<UUID, TimeoutMessage>,
    sent_responded: Vec<(ServerMessageContent, Response)>,
    to_retry: Vec<ServerMessage>,
}
impl ComsManager {
    pub fn received(&mut self, msg: ServerMessage) {
        match msg.content {
            super::ServerMessageContent::RESPONSE(response) => self.handle_response(msg.uuid, response),
            content => self.handle_received(msg.uuid, content),
        };
    }
    
    pub fn sent(&mut self, msg: ServerMessage) {
        match msg.content {
            ServerMessageContent::RESPONSE(_) => (),
            content => self.handle_sent(msg.uuid, content),
        }
    }

    pub fn update(&mut self) {
        let mut to_retry = Vec::new();

        self.sent_waiting
            .retain(|uuid, msg| {
                if msg.arrival.elapsed() > Duration::from_millis(100) {
                    to_retry.push(ServerMessage::new(*uuid, msg.content.clone()));
                    
                    return false;
                }

                true
            });
        
        self.to_retry.append(&mut to_retry);
    }
    
    pub fn to_retry(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.to_retry)
    }
    
    pub fn sent_responded(&mut self) -> Vec<(ServerMessageContent, Response)> {
        std::mem::take(&mut self.sent_responded)
    }

    pub fn received_waiting(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.received_waiting)
            .drain()
            .map(|(uuid, content)| ServerMessage::new(uuid, content))
            .collect()
    }
}
impl ComsManager {
    fn handle_response(&mut self, msg_uuid: UUID, response: Response) {
        if let Some(sent) = self.sent_waiting.remove(&msg_uuid) {
            self.sent_responded.push((sent.content, response));
        }
    }
    
    fn handle_received(&mut self, msg_uuid: UUID, content: ServerMessageContent) {
        self.received_waiting.insert(msg_uuid, content);
    }

    fn handle_sent(&mut self, msg_uuid: UUID, content: ServerMessageContent) {
        self.sent_waiting.insert(
            msg_uuid,
            TimeoutMessage {
                arrival: Instant::now(),
                content,
            }
        );
    }
}