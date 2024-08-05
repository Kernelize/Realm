use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub user: String,
    pub date: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messages {
    pub messages: Vec<Message>,
}

pub type RoomStore = HashMap<String, VecDeque<Message>>;

#[derive(Clone, Default)]
pub struct MessageStore {
    pub messages: Arc<RwLock<RoomStore>>,
}

impl MessageStore {
    pub async fn insert(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_owned()).or_default();
        messages.push_front(message);
        messages.truncate(20);
    }

    pub async fn get(&self, room: &str) -> Vec<Message> {
        let messages = self.messages.read().await.get(room).cloned();
        messages.unwrap_or_default().into_iter().rev().collect()
    }
}