// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            return Err("Only `To-Do`, `In Progress`, and `Done` statuses are allowed".to_string());
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, new_title: String) -> Result<(), String> {
        if new_title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if new_title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        self.title = new_title;
        Ok(())
    }

    pub fn set_description(&mut self, new_description: String) -> Result<(), String> {
        if new_description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if new_description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }
        self.description = new_description;
        Ok(())
    }

    pub fn set_status(&mut self, new_status: String) -> Result<(), String> {
        if new_status != "To-Do" && new_status != "In Progress" && new_status != "Done" {
            return Err("Only `To-Do`, `In Progress`, and `Done` statuses are allowed".to_string());
        }
        self.status = new_status;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket.set_title("A new title".into()).unwrap();
        ticket.set_description("A new description".into()).unwrap();
        ticket.set_status("Done".into()).unwrap();

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket.set_title("".into()).expect("Title cannot be empty");
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket
            .set_description("".into())
            .expect("Description cannot be empty");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket
            .set_title(overly_long_title())
            .expect("Title cannot be longer than 50 bytes");
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket
            .set_description(overly_long_description())
            .expect("Description cannot be longer than 500 bytes");
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), "To-Do".into()).unwrap();
        ticket
            .set_status("Funny".into())
            .expect("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
    }
}
