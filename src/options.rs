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
use std::os::raw::c_void;
use std::ptr;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWND(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HINSTANCE(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct COLORREF(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Onhold,
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
#[derive(Debug, Clone, Copy, Default)]
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
pub const FLAG_ALLOW_CHANGE_PASSPHRASE: i32 = 1 << 1;
pub const FLAG_SAVE_KEY_PASS: i32 = 1 << 4;
pub const FLAG_SAVE_AUTH_PASS: i32 = 1 << 5;
pub const FLAG_DISABLE_SAVE_PASS: i32 = 1 << 6;
pub const FLAG_DISABLE_ECHO_MSG: i32 = 1 << 7;
pub const FLAG_DAEMON_PERSISTENT: i32 = 1 << 8;
pub const FLAG_WAIT_UNLOCK: i32 = 1 << 9;
pub const FLAG_CONFIG_DISABLED: i32 = 1 << 10;

#[derive(Debug)]
pub struct Options {
    pub auto_connect: Vec<String>,
    pub num_configs: i32,
    pub service_state: ServiceState,
    pub proxy_source: ProxySource,
    pub proxy_type: ProxyType,
    pub exe_path: String,
    pub install_path: String,
    pub config_dir: String,
    pub log_dir: String,
    pub ext_string: String,
    pub silent_connection: u32,
    pub version: Version,
    pub ovpn_version: Version,
    pub dpi_scale: u32,
    pub clr_warning: COLORREF,
    pub clr_error: COLORREF,
    pub h_wnd: HWND,
    pub h_instance: HINSTANCE,
    pub netcmd_semaphore: HANDLE,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            auto_connect: Vec::new(),
            num_configs: 0,
            service_state: ServiceState::Disconnected,
            proxy_source: ProxySource::Config,
            proxy_type: ProxyType::Http,
            exe_path: String::new(),
            install_path: String::new(),
            config_dir: String::new(),
            log_dir: String::new(),
            ext_string: "ovpn".to_string(),
            silent_connection: 0,
            version: Version::default(),
            ovpn_version: Version::default(),
            dpi_scale: 100,
            clr_warning: COLORREF(0x000000FF), // RGB(0xff, 0, 0)
            clr_error: COLORREF(0x000000FF),   // RGB(0xff, 0, 0)
            h_wnd: HWND(ptr::null_mut()),
            h_instance: HINSTANCE(ptr::null_mut()),
            netcmd_semaphore: HANDLE(ptr::null_mut()),
        }
    }
}

pub fn init_options() -> Options {
    Options::default()
}
