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
