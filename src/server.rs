use ptyprocess::PtyProcess;
use std::{
    io::prelude::*,
    io::BufReader,
    net::{TcpListener, TcpStream},
    process::{Command, Stdio},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 256];

    while stream.read(&mut buf).unwrap() != 0 {
        dbg!(&buf);
    }

    let mut cmd = String::from_utf8_lossy(&buf);

    println!("{cmd:?}");
    let mut cmd = cmd.trim().trim_matches('\0').split_ascii_whitespace();
    let bin = cmd.next().unwrap();

    let args = cmd.collect::<Vec<_>>();

    dbg!(&bin, &args);

    let mut command = Command::new(bin);
    command.args(args);

    let mut process = PtyProcess::spawn(command).unwrap();

    let mut output = process.get_raw_handle().unwrap();

    let mut output_reader = BufReader::new(output);
    for line in output_reader.lines() {
        dbg!(&line);
        stream.write(line.unwrap().as_bytes()).unwrap();
        stream.write(b"\n").unwrap();
    }
}
