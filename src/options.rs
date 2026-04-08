pub const MAX_PATH: usize = 260;
pub const UNLEN: usize = 256;
pub const MAX_NAME: usize = UNLEN + 1;

// Mocking windows types for Linux-based porting verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HANDLE(pub isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HWND(pub isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SOCKET(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: [u8; 4],
    pub sin_zero: [u8; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Overlapped {
    pub internal: usize,
    pub internal_high: usize,
    pub offset: u32,
    pub offset_high: u32,
    pub h_event: HANDLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colorref(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ServiceState {
    NoAccess = -1,
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxySource {
    Config,
    Windows,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyType {
    Http,
    Socks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    Disconnected,
    OnHold,
    Connecting,
    Reconnecting,
    Connected,
    Disconnecting,
    Suspending,
    Suspended,
    Resuming,
    Detaching,
    Detached,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub release: i32,
    pub stage: i32,
}

#[derive(Debug, Clone)]
pub struct ConfigGroup {
    pub id: i32,
    pub name: String,
    pub parent: i32,
    pub active: bool,
    pub children: i32,
    pub pos: i32,
    pub menu: isize,
}

#[derive(Debug, Clone)]
pub struct ServiceIo {
    pub overlapped: Overlapped,
    pub pipe: HANDLE,
    pub event: HANDLE,
    pub read_buf: [u16; 512],
}

pub struct Connection {
    pub config_file: String,
    pub config_name: String,
    pub config_dir: String,
    pub log_path: String,
    pub ip: String,
    pub ipv6: String,
    pub auto_connect: bool,
    pub state: ConnState,
    pub failed_psw_attempts: i32,
    pub failed_auth_attempts: i32,
    pub connected_since: i64,
    pub proxy_type: ProxyType,
    pub group: i32,
    pub pos: i32,

    pub manage: Manage,

    pub h_process: HANDLE,
    pub iserv: ServiceIo,

    pub exit_event: HANDLE,
    pub thread_id: u32,
    pub hwnd_status: HWND,
    pub flags: i32,
    pub dynamic_cr: Option<String>,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub daemon_state: String,
    pub id: i32,
    pub next: *mut Connection,
}

pub struct Manage {
    pub sk: SOCKET,
    pub skaddr: SockaddrIn,
    pub timeout: i64,
    pub password: [u8; 4096],
    pub saved_data: Option<Vec<u8>>,
    pub connected: u32,
}

pub struct Options {
    pub auto_connect: Vec<String>,
    pub connections: Vec<Connection>,
    pub groups: Vec<ConfigGroup>,
    pub service_state: ServiceState,

    pub proxy_source: ProxySource,
    pub proxy_type: ProxyType,
    pub proxy_http_address: String,
    pub proxy_http_port: String,
    pub proxy_socks_address: String,
    pub proxy_socks_port: String,

    pub exe_path: String,
    pub install_path: String,
    pub global_config_dir: String,
    pub config_auto_dir: String,
    pub global_log_dir: String,
    pub priority_string: String,
    pub ovpn_admin_group: String,
    pub disable_save_passwords: u32,
    pub auth_pass_concat_otp: u32,

    pub config_dir: String,
    pub ext_string: String,
    pub log_dir: String,
    pub log_append: u32,
    pub log_viewer: String,
    pub editor: String,
    pub silent_connection: u32,
    pub iservice_admin: u32,
    pub show_balloon: u32,
    pub show_script_window: u32,
    pub connectscript_timeout: u32,
    pub disconnectscript_timeout: u32,
    pub preconnectscript_timeout: u32,
    pub config_menu_view: u32,
    pub disable_popup_messages: u32,
    pub popup_mute_interval: u32,
    pub mgmt_port_offset: u32,

    pub ovpn_engine: u32,
    pub enable_persistent: u32,
    pub enable_auto_restart: u32,
    pub disable_password_reveal: u32,

    pub h_wnd: HWND,
    pub h_instance: HANDLE,
    pub session_locked: bool,
    pub netcmd_semaphore: HANDLE,
    pub version: Version,
    pub ovpn_version: Version,
    pub ovpn_version_str: String,
    pub dpi_scale: u32,
    pub clr_warning: Colorref,
    pub clr_error: Colorref,
    pub action: i32,
    pub action_arg: String,
    pub session_semaphore: HANDLE,
    pub event_log: HANDLE,
    pub use_qr_for_url: u32,
}
