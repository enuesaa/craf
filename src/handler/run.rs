use std::{process::{Command, Stdio}};
use crate::service::registry::Registry;

/**
 *
 * see https://keens.github.io/blog/2016/12/02/rustnopurosesu/
 * see https://www.howtosolutions.net/2021/10/rust-creating-simple-terminal-interactive-input-prompt-app/
 * see https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results
 * see https://stackoverflow.com/questions/66060139/how-to-tee-stdout-stderr-from-a-subprocess-in-rust
 */
pub fn run(name: &str) {
    let registry = Registry::new();
    let commanddef = registry.get_command(name);

    let mut child = Command::new(commanddef.bin)
        .args(commanddef.args)
        .spawn()
        .unwrap();

    let res = child.wait().unwrap();
    println!("{:?}", res);

    // コマンドを実行して標準出力を受け取る
    // let output = Command::new("echo")
    //     .args(["aaa"])
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .output()
    //     .unwrap();
    // println!("{:?}", output); // Output { status: ExitStatus(unix_wait_status(0)), stdout: "aaa\n", stderr: "" }
    // println!("{}", String::from_utf8_lossy(&output.stdout)); // aaa

    // 子プロセスの標準出力を、そのまま出す
    // https://doc.rust-lang.org/std/process/struct.Stdio.html#method.inherit
    // Command::new("echo")
    //     .args(["aaa"])
    //     .stdout(Stdio::inherit())
    //     .output()
    //     .unwrap();
    // println!("bbb");
    // aaa
    // bbb

    // Command::new("node")
    //     .stdin(Stdio::inherit())
    //     .stdout(Stdio::inherit())
    //     .stderr(Stdio::inherit())
    //     .output()
    //     .unwrap();
    // println!("hello from rust");

    // Command::new("sleep")
    //     .args(["10"])
    //     .stdout(Stdio::inherit())
    //     .output() // コマンド実行中は待機する
    //     .unwrap();

    // 非同期に実行
    // 先に bbb が出力される
    // Command::new("echo")
    //     .args(["aaa"])
    //     .spawn()
    //     .unwrap();
    // println!("bbb");
    // bbb
    // aaa

    // let mut child = Command::new("echo")
    //     .args(["aaa"])
    //     .spawn()
    //     .unwrap();
    // let status = child.wait().unwrap();
    // println!("{:?}", status); // ExitStatus(unix_wait_status(0))

    // let child = Command::new("echo")
    //     .args(["aaa"])
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()
    //     .unwrap();
    // let output = child.wait_with_output().unwrap();
    // println!("{:?}", output); // Output { status: ExitStatus(unix_wait_status(0)), stdout: "aaa\n", stderr: "" }
    // println!("{}", String::from_utf8_lossy(&output.stdout)); // aaa

    // let mut child = Command::new("node")
    //     .stdin(Stdio::inherit())
    //     .stdout(Stdio::inherit())
    //     .stderr(Stdio::inherit())
    //     .spawn()
    //     .unwrap();
    // child.wait().unwrap();
    // println!("hello from rust");
    // $ cargo run run --name a
    // Welcome to Node.js v18.16.0.
    // Type ".help" for more information.
    // > new Date()
    // 2023-06-11T16:38:11.155Z
    // > .exit
    // hello from rust
}
