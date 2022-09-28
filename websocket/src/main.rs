use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let (mut client, _response) = connect_async("ws://echo.websocket.events").await.unwrap();
    let (mut sender, mut reciever) = client.split();
    let handle = tokio::spawn(async {
        loop {
            let msg = reciever.read_message().await;
            println!("{:?}", msg);
        }
    });
    println!("{:?}", client);
}
