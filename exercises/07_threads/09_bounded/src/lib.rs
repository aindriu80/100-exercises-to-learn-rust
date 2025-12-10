// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();

        self.sender.send(Command::Insert {
            draft,
            response_channel: response_sender,
        }).map_err(|e| e.to_string())?;
        
        // Receive the response - we can unwrap here because the server
        // will always send a response before closing the channel
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();

        self.sender.send(Command::Get {
            id,
            response_channel: response_sender,
        }).map_err(|e| e.to_string())?;
        
        // Receive the response
        Ok(response_receiver.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                // Send the response back to the client
                // Ignore the error if the client has disconnected
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).cloned();
                // Send the response back to the client
                let _ = response_channel.send(ticket);
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
