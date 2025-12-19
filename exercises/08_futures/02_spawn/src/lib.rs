use std::io::Result as IoResult;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    // keep both listeners alive forever.
    loop {
        // wait on eithe rlistener. `tokio::select!` lets us accept from both
        // concurrently witout blocking the other one.
        tokio::select! {
            // Accept from the first listener.
            res = first.accept() => {
                let (socket, _) = res?; // propagate accept errors
                tokio::spawn(handle_echo(socket));
            }

            // Accept from the second listener
            res = second.accept() => {
                let (socket, _) = res?;
                tokio::spawn(handle_echo(socket));
            }
        }
    }
}
// handle a single client: read everything until EOF, then write the same bytes
// back to the client.  Errors are ignored - the test only cares that the echo  works.
async fn handle_echo(mut socket: TcpStream) {
    let mut data = Vec::new();
    let mut read_buf = [0u8; 1024];

    // Read until the client closes the write side.
    loop {
        match socket.read(&mut read_buf).await {
            Ok(0) => break, // EOF
            Ok(n) => data.extend_from_slice(&read_buf[..n]),
            Err(_) => return, // drop on read error
        }
    }

    // Echo the data back.  Errors are ignored - the test only check that the
    // data matches
    let _ = socket.write_all(&data).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
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
