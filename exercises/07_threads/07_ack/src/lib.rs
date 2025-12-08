use crate::data::{Ticket, TicketDraft};
use crate::store::TicketId;
use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();

    for command in receiver {
        match command {
            Command::Insert {
                draft,
                response_sender,
            } => {
                let id = store.add_ticket(draft);
                // We ignore send errors - the client dropped the channel.
                let _ = response_sender.send(id);
            }
            Command::Get {
                id,
                response_sender,
            } => {
                let ticket_opt = store.get(id).cloned();
                let _ = response_sender.send(ticket_opt);
            }
        }
    }
}
