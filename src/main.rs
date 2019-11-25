use futures_util::future;

use tokio::net::process::Command;

#[tokio::main]
async fn main() {
    let cmd = Command::new("echo")
        .arg("hello world")
        .status();

    future::select(future::pending(), cmd).await;
}
