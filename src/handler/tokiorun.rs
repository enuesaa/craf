use tokio::process::Command;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use std::io;

pub async fn tokiorun() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    tokio::spawn(async move {
        println!("{:?}", stdout);
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed");
        // stdin.write_all(b"hello world\n").await.unwrap();
        stdin.write_all(line.as_bytes()).await.unwrap();
        drop(stdin);
    });

    let _ = child.wait().await;
    Ok(())
}