use crate::{
    error::{Error, Result},
    messages::{Message, MessageQuery, MsgId, MsgRef},
    users::UserAuth,
    Identity,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

type MsgTree<K, V> = Arc<RwLock<BTreeMap<K, V>>>;

/// A query object that get's built and then executed asynchronously
pub(crate) struct StoreQuery<'store> {
    store: &'store MsgStore,
    user: Identity,
    unread: bool,
    service: Option<String>,
    query: Option<MessageQuery>,
    limit: Option<usize>,
}

impl<'store> StoreQuery<'store> {
    /// Filter messages for unreads only
    pub(crate) fn unread(self) -> Self {
        Self {
            unread: true,
            ..self
        }
    }

    /// Filter messages by association with a service
    ///
    /// This lookup uses message ispection and might be generally
    /// slower than others.
    pub(crate) fn service<S>(self, service: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            service: Some(service.into()),
            ..self
        }
    }

    pub(crate) fn limit(self, limit: usize) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    /// Filter messages additionally with a user provided query
    pub(crate) fn constraints(self, query: MessageQuery) -> Self {
        Self {
            query: Some(query),
            ..self
        }
    }

    /// Execute the query against the store
    pub(crate) fn exec(self) -> Result<Vec<MsgRef>> {
        let StoreQuery {
            store,
            user,
            query,
            unread,
            service,
            limit,
        } = self;

        store
            .by_user
            .write()
            .unwrap()
            .get_mut(&user)
            .map_or(Ok(vec![]), |set| {
                Ok(set
                    .iter_mut()
                    // Conditional filters that are applied only if the query matches
                    .filter(|msg| if unread { msg.unread() } else { true })
                    .filter(|msg| {
                        if let Some(ref s) = service {
                            &msg.inner().associator == s
                        } else {
                            true
                        }
                    })
                    .filter(|msg| match query {
                        Some(MessageQuery::Id(ref id)) => &msg.inner().id == id,
                        Some(MessageQuery::Sender(ref sender)) => &msg.inner().sender == sender,
                        None => true,
                    })
                    .take(limit.unwrap_or(usize::max_value()))
                    .map(|msg| msg.read())
                    .collect())
            })
    }
}

/// Encodes the read-state of a Message
///
/// The state is transformed when a query yields in this message being
/// returned to an endpoint. At no point can the internal message
/// store keep track if a message has actually been shown to a
/// human. As such, the accuracy of this data might be flawed.
#[derive(Clone)]
pub(crate) enum MsgState {
    /// A previously read message
    Read(MsgRef),
    /// An unread message
    Unread(MsgRef),
}

impl MsgState {
    /// Change state to read, while returning reference to inner message data
    fn read(&mut self) -> MsgRef {
        let msg = match self {
            Self::Unread(msg) => msg,
            Self::Read(msg) => msg,
        };

        let msg_ref = Arc::clone(&msg);
        *self = Self::Read(Arc::clone(&msg));
        msg_ref
    }

    fn unread(&self) -> bool {
        match self {
            Self::Unread(_) => true,
            _ => false,
        }
    }

    fn inner(&self) -> &MsgRef {
        match self {
            Self::Unread(msg) => msg,
            Self::Read(msg) => msg,
        }
    }
}

/// A searchable index of messages encountered by this system
#[derive(Clone)]
pub(crate) struct MsgStore {
    /// Owns Message references by their ID
    by_id: MsgTree<MsgId, MsgState>,
    /// By-user reference table to improve search performance
    by_user: MsgTree<Identity, Vec<MsgState>>,
}

impl MsgStore {
    pub(crate) fn new() -> Self {
        Self {
            by_id: Arc::new(RwLock::new(BTreeMap::default())),
            by_user: Arc::new(RwLock::new(BTreeMap::default())),
        }
    }

    /// Start a query for a given user
    pub(crate) fn query(&self, user: Identity) -> StoreQuery {
        StoreQuery {
            user,
            store: self,
            unread: false,
            service: None,
            query: None,
            limit: None,
        }
    }

    /// Permanently store a whole Message
    ///
    /// For this function it doesn't matter if the `Message` was
    /// dispatched by this system or has come in from outside.
    pub(crate) fn insert(&self, user: Identity, msg: MsgState) {
        let id = msg.inner().id.clone();

        self.by_user
            .write()
            .expect("Failed to lock MsgStore!")
            .entry(user)
            .or_insert(vec![])
            .push(msg.clone());

        self.by_id
            .write()
            .expect("Failed to lock MsgStore!")
            .insert(id, msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::{Message, MsgId, MsgState, MsgStore, Recipient, SigTrust};
    use crate::{utils, Identity};
    use std::sync::Arc;

    fn setup(id: Identity) -> MsgStore {
        let store = MsgStore::new();
        let msg = Message {
            id: MsgId::random(),
            sender: Identity::truncate(&utils::random(16)),
            associator: "__test".into(),
            sign: SigTrust::Unverified,
            payload: vec![1, 3, 1, 2],
        };
        store.insert(id, MsgState::Read(Arc::new(msg)));
        store
    }

    #[test]
    fn simple() {
        let id = Identity::truncate(&utils::random(16));
        let store = setup(id);
        assert!(store.query(id).exec().unwrap().len() > 0);
    }
}
