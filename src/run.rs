use std::process::Command;
use std::io::Result;

pub fn run() -> Result<()> {
    let mut child = Command::new("node")
        // .args(["--version"])
        .spawn()
        .unwrap();
    let a = child.wait().unwrap();
    println!("{:?}", a);
    // let stdin = child.stdin.take().unwrap();
    // let stdout = child.stdout.take().unwrap();
    // println!("{:?}", stdin);
    // println!("{:?}", stdout);
    Ok(())
}
