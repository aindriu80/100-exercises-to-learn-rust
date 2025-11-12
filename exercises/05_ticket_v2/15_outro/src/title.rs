// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.
//
use std::convert::TryFrom;
use std::fmt;

/// Public new‑type wrapping a String.
#[derive(Debug, PartialEq, Clone)] // <- needed for Ticket::Debug / PartialEq
pub struct TicketTitle(String);

/// Error type used by the TryFrom implementations.
#[derive(Debug, PartialEq)]
pub enum TitleError {
    Empty,
    TooLong { limit: usize },
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "The title cannot be empty"),
            TitleError::TooLong { limit } => {
                write!(f, "The title cannot be longer than {} bytes", limit)
            }
        }
    }
}

impl std::error::Error for TitleError {}

const TITLE_MAX: usize = 50;

/// TryFrom<String>
impl TryFrom<String> for TicketTitle {
    type Error = TitleError; // <-- the missing piece

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TitleError::Empty);
        }
        if value.len() > TITLE_MAX {
            return Err(TitleError::TooLong { limit: TITLE_MAX });
        }
        Ok(TicketTitle(value)) // <-- use `Ok`, not `OK`
    }
}

/// TryFrom<&str>
impl<'a> TryFrom<&'a str> for TicketTitle {
    type Error = TitleError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        TicketTitle::try_from(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
