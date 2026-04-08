mod options;
mod config_parser;
mod misc;
mod manage;
mod openvpn;

fn main() {
    println!("OpenVPN GUI in Rust");
mod config_parser;
mod options;
mod misc;
mod gui;

use gui::OpenVpnGui;

fn main() -> iced::Result {
    iced::application(
        OpenVpnGui::new,
        OpenVpnGui::update,
        OpenVpnGui::view,
    )
    .run()
}
