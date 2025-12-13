// TODO: Replace `Mutex` with `RwLock` in the `TicketStore` struct and
//  all other relevant places to allow multiple readers to access the ticket store concurrently.
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

/// Opaque client that talks to the background server via a channel.
#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    /// Insert a new ticket.  Returns the new ticket ID.
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    /// Get a ticket by ID.  Returns an `Arc<RwLock<Ticket>>` so the caller can read or
    /// modify the ticket.  The `Option` is `None` if the ID is unknown.
    pub fn get(
        &self,
        id: TicketId,
    ) -> Result<Option<Arc<std::sync::RwLock<Ticket>>>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }
}

/// Error returned when the channel buffer is full.
#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

/// Launch a new ticket‑store server with a bounded channel capacity.
/// Returns a client that can be used to send requests.
pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Arc<std::sync::RwLock<Ticket>>>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket);
            }
            Err(_) => {
                // No more senders – shutdown the server.
                break;
            }
        }
    }
}
