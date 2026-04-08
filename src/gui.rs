use iced::widget::{button, column, row, text, container};
use iced::{Alignment, Element, Length, Task};
use crate::config_parser::{ConfigEntry, config_parse};
use crate::options::{Options, init_options};

#[derive(Debug, Clone)]
pub enum Message {
    ParseConfig,
    Connect(String),
}

pub struct OpenVpnGui {
    _options: Options,
    pub configs: Vec<ConfigEntry>,
    pub status: String,
}

impl OpenVpnGui {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                _options: init_options(),
                configs: Vec::new(),
                status: "Ready".to_string(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ParseConfig => {
                self.status = "Parsing config...".to_string();
                let path = "sample.ovpn";

                if !std::path::Path::new(path).exists() {
                     std::fs::write(path, "remote 1.2.3.4 1194 udp\ndev tun\n").ok();
                }

                match config_parse(path) {
                    Ok(entries) => {
                        self.configs = entries;
                        self.status = format!("Loaded {} entries", self.configs.len());
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                    }
                }
            }
            Message::Connect(name) => {
                self.status = format!("Connecting to {}...", name);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("OpenVPN GUI").size(40);

        let status_text = text(&self.status).size(20);

        let mut configs_col = column![].spacing(10);
        for (i, config) in self.configs.iter().enumerate() {
            if !config.tokens.is_empty() {
                let config_name = format!("Config {}", i + 1);
                configs_col = configs_col.push(
                    row![
                        text(format!("{:?}", config.tokens)),
                        button("Connect").on_press(Message::Connect(config_name)),
                    ]
                    .spacing(20)
                    .align_y(Alignment::Center),
                );
            }
        }

        let content = column![
            title,
            button("Refresh Configs").on_press(Message::ParseConfig),
            status_text,
            configs_col,
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
