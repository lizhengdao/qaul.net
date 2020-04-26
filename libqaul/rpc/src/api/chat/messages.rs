use super::ChatRpc;
use async_std::sync::Arc;
use async_trait::async_trait;
use futures::{future::FutureExt, stream::Stream};
use libqaul::{error::Result, users::UserAuth};
use qaul_chat::{Chat, ChatMessage, RoomId, Subscription};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscribe {
    pub auth: UserAuth,
    pub room: RoomId,
}

#[async_trait]
impl ChatRpc for Subscribe {
    type Response = Result<Subscription>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.subscribe(self.auth, self.room).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Get {
    pub auth: UserAuth,
    pub room: RoomId,
}

#[async_trait]
impl ChatRpc for Get {
    type Response = Result<Vec<ChatMessage>>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.load_messages(self.auth, self.room).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Send {
    pub auth: UserAuth,
    pub room: RoomId,
    pub text: String,
}

#[async_trait]
impl ChatRpc for Send {
    type Response = Result<()>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.send_message(self.auth, self.room, self.text).await
    }
}
