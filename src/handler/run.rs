use std::process::Command;
use std::io::Result;

pub fn run() -> Result<()> {
    let child = Command::new("node")
        .spawn()
        .unwrap();

    let res = child
        .wait_with_output()
        .unwrap();
    println!("{:?}", res);

    // https://keens.github.io/blog/2016/12/02/rustnopurosesu/
    // https://www.howtosolutions.net/2021/10/rust-creating-simple-terminal-interactive-input-prompt-app/
    // https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results
    // https://stackoverflow.com/questions/66060139/how-to-tee-stdout-stderr-from-a-subprocess-in-rust

    Ok(())
}
