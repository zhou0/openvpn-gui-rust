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

#[derive(Debug, Clone, Copy, Default)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub release: i32,
    pub stage: i32,
}

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
