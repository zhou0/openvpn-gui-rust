use crate::options::Connection;
use std::sync::Arc;
use std::net::TcpStream;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgmtRtMsgType {
    Ready,
    Stop,
    Bytecount,
    Echo,
    Hold,
    Log,
    Password,
    Proxy,
    State,
    NeedOk,
    NeedStr,
    Pkcs11IdCount,
    InfoMsg,
    Timeout,
    Validate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgmtCmdType {
    Regular,
    Combined,
}

pub type MgmtMsgFunc = Arc<dyn Fn(&mut Connection, &str) + Send + Sync>;

pub struct MgmtCmd {
    pub command: String,
    pub handler: Option<MgmtMsgFunc>,
    pub cmd_type: MgmtCmdType,
}

pub struct ManagementClient {
    stream: Option<TcpStream>,
    cmd_queue: Vec<MgmtCmd>,
    saved_data: Vec<u8>,
}

impl ManagementClient {
    pub fn new() -> Self {
        Self {
            stream: None,
            cmd_queue: Vec::new(),
            saved_data: Vec::new(),
        }
    }

    pub fn open(&mut self, addr: &str) -> std::io::Result<()> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send_command(&mut self, cmd: MgmtCmd) -> std::io::Result<()> {
        if let Some(ref mut stream) = self.stream {
            let mut buf = cmd.command.clone();
            buf.push('\n');
            stream.write_all(buf.as_bytes())?;
            self.cmd_queue.push(cmd);
        }
        Ok(())
    }

    pub fn process_input(&mut self, conn: &mut Connection) -> std::io::Result<()> {
        if let Some(ref mut stream) = self.stream {
            let mut buf = [0u8; 1024];
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    self.saved_data.extend_from_slice(&buf[..n]);
                    self.handle_data(conn);
                }
                Ok(_) => {}
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn handle_data(&mut self, conn: &mut Connection) {
        while let Some(pos) = self.saved_data.iter().position(|&b| b == b'\n') {
            let line_bytes = self.saved_data.drain(..pos + 1).collect::<Vec<u8>>();
            if let Ok(line) = std::str::from_utf8(&line_bytes) {
                let line = line.trim();
                self.handle_line(conn, line);
            }
        }
    }

    fn handle_line(&mut self, conn: &mut Connection, line: &str) {
        if line.starts_with('>') {
            let content = &line[1..];
            println!("Management Notification: {}", content);
        } else if !self.cmd_queue.is_empty() {
            let cmd = self.cmd_queue.remove(0);
            if let Some(handler) = cmd.handler {
                handler(conn, line);
            }
        }
    }

    pub fn close(&mut self) {
        self.stream = None;
        self.cmd_queue.clear();
        self.saved_data.clear();
    }
}
