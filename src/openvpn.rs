use crate::options::{Connection, ConnState};

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

pub fn daemon_state_resid(name: &str) -> i32 {
    match name {
        "CONNECTING" => 100,
        "WAIT" => 101,
        "AUTH" => 102,
        "GET_CONFIG" => 103,
        "ASSIGN_IP" => 104,
        "ADD_ROUTES" => 105,
        "CONNECTED" => 106,
        "RECONNECTING" => 107,
        "EXITING" => 108,
        "RESOLVE" => 109,
        "TCP_CONNECT" => 110,
        _ => 0,
    }
}

pub fn on_ready(conn: &mut Connection, _msg: &str) {
    println!("Management interface ready for connection: {}", conn.config_name);
}

pub fn on_state_change(conn: &mut Connection, data: &str) {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() < 3 {
        return;
    }

    let state = parts[1];
    conn.daemon_state = state.to_string();
    println!("Connection {} state changed to {}", conn.config_name, state);

    if state == "CONNECTED" {
        conn.state = ConnState::Connected;
    }
}

pub fn on_log_line(_conn: &mut Connection, line: &str) {
    println!("OpenVPN Log: {}", line);
}
