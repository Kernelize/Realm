use chrono::{DateTime, Utc};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::models::message::{Message, MessageStore, Messages};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageIn {
    room: String,
    text: String,
}

fn on_connect(socket: SocketRef) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room), store: State<MessageStore>| async move {
            info!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let messages = store.get(&room).await;
            let _ = socket.emit("messages", Messages { messages });
        },
    );

    socket.on(
        "message",
        |socket: SocketRef, Data::<MessageIn>(data), store: State<MessageStore>| async move {
            info!("Received message: {:?}", data);

            let response = Message {
                text: data.text,
                user: format!("anon-{}", socket.id),
                date: Utc::now(),
            };

            store.insert(&data.room, response.clone()).await;

            let _ = socket.within(data.room).emit("message", response);
        },
    );
}

pub fn make_router() -> Router {
    let messages = crate::models::message::MessageStore::default();
    let (layer, io) = SocketIo::builder().with_state(messages).build_layer();
    let layer = ServiceBuilder::new()
        .layer(CorsLayer::permissive())
        .layer(layer);

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);
    let layer = layer.compat();
    Router::with_path("socket.io").hoop(layer).goal(hello)
}
