// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    // Share the reply with both loops
    let reply = Arc::new(reply);
    let first_reply = reply.clone();
    let second_reply = reply.clone();

    // ----- Connection‑handling loop for the *first* listener -----
    let first_loop = async move {
        loop {
            // Accept a client
            let (mut socket, _) = first.accept().await.expect("accept failed");

            // Convert the reply to bytes once per connection
            let msg = format!("{}", &*first_reply).into_bytes();

            // Send the reply, then drop the socket (connection closed)
            socket.write_all(&msg).await.expect("write failed");
        }
    };

    // ----- Connection‑handling loop for the *second* listener -----
    let second_loop = async move {
        loop {
            let (mut socket, _) = second.accept().await.expect("accept failed");
            let msg = format!("{}", &*second_reply).into_bytes();
            socket.write_all(&msg).await.expect("write failed");
        }
    };

    // Run the two loops concurrently. The `join!` macro expands to an
    // async block that returns a tuple, so nothing can run after it,
    // which is why there is no code after the call.
    tokio::join!(first_loop, second_loop);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
