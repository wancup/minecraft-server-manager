#![windows_subsystem = "windows"]

use crate::server::{post_request_to_server, PostRequestType, ResponsePayload, ServerState};
use anyhow::Result;
use iced::{
    alignment, button, clipboard, executor, window, Alignment, Application, Button, Color, Column,
    Command, Element, Length, Row, Settings, Text,
};
use std::time::Duration;

mod config;
mod server;

#[derive(Debug, Default)]
struct MsmClient {
    server_info: ResponsePayload,
    err_message: String,
    state_checked_count: usize,
    copy_address_button: button::State,
    start_server_button: button::State,
    reload_server_state_button: button::State,
    stop_server_button: button::State,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    StartServer,
    CopyServerAddress,
    ReloadServerState,
    StopServer,
    UpdateServerState(Result<ResponsePayload, String>),
    AddressInputIsChanged(String),
}

impl MsmClient {
    fn get_next_command_after_update_state(
        &mut self,
        server_info: ResponsePayload,
    ) -> Command<Message> {
        let next_command = match server_info.server_state {
            ServerState::Pending | ServerState::Stopping => {
                self.state_checked_count += 1;
                Command::perform(
                    post_request_to_server(
                        PostRequestType::GetServerStatus,
                        Some(Duration::from_millis(1500)),
                    ),
                    Message::UpdateServerState,
                )
            }
            _ => {
                self.state_checked_count = 0;
                Command::none()
            }
        };
        self.server_info = server_info;
        next_command
    }

    fn perform_start_server_command(&mut self) -> Command<Message> {
        match self.server_info.server_state {
            ServerState::Stopped => {
                self.server_info = ResponsePayload {
                    server_state: ServerState::Connecting,
                    ip_address: self.server_info.ip_address.clone(),
                };
                Command::perform(
                    post_request_to_server(PostRequestType::StartServer, None),
                    Message::UpdateServerState,
                )
            }
            _ => {
                self.err_message = "Server is not stopped".to_string();
                Command::none()
            }
        }
    }

    fn perform_stop_server_command(&mut self) -> Command<Message> {
        match self.server_info.server_state {
            ServerState::Running => {
                self.server_info = ResponsePayload {
                    server_state: ServerState::Connecting,
                    ip_address: self.server_info.ip_address.clone(),
                };
                Command::perform(
                    post_request_to_server(PostRequestType::StopServer, None),
                    Message::UpdateServerState,
                )
            }
            _ => {
                self.err_message = "Server is not running".to_string();
                Command::none()
            }
        }
    }
}

impl Application for MsmClient {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            MsmClient::default(),
            Command::perform(
                post_request_to_server(PostRequestType::GetServerStatus, None),
                Message::UpdateServerState,
            ),
        )
    }

    fn title(&self) -> String {
        "Minecraft Server Manager".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::StartServer => self.perform_start_server_command(),
            Message::CopyServerAddress => {
                if let Some(address) = self.server_info.ip_address.clone() {
                    clipboard::write::<Self::Message>(address);
                };
                Command::none()
            }
            Message::ReloadServerState => {
                self.server_info = ResponsePayload {
                    server_state: ServerState::Connecting,
                    ip_address: self.server_info.ip_address.clone(),
                };
                Command::perform(
                    post_request_to_server(PostRequestType::GetServerStatus, None),
                    Message::UpdateServerState,
                )
            }
            Message::StopServer => self.perform_stop_server_command(),
            Message::UpdateServerState(update) => match update {
                Ok(server_info) => self.get_next_command_after_update_state(server_info),
                Err(err) => {
                    self.err_message = "Err: ".to_string() + &err;
                    Command::none()
                }
            },
            Message::AddressInputIsChanged(ref _change) => Command::none(), //NOP
        }
    }

    fn view(&mut self) -> Element<Message> {
        let api_command_button = |state, label, message| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .on_press(message)
        };

        let checking_counter_str = ".".repeat(self.state_checked_count);
        Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Alignment::Center)
            .push(
                Text::new(&self.server_info.server_state.to_string())
                    .color(self.server_info.server_state.color())
                    .size(30),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .push(Text::new("Server Address:"))
                    .spacing(10)
                    .push(Text::new(
                        &self.server_info.ip_address.clone().unwrap_or_default(),
                    ))
                    .push(
                        Button::new(&mut self.copy_address_button, Text::new("Copy"))
                            .height(Length::Shrink)
                            .on_press(Message::CopyServerAddress),
                    ),
            )
            .push(Text::new(checking_counter_str).size(30))
            .push(
                Row::new()
                    .spacing(8)
                    .push(api_command_button(
                        &mut self.start_server_button,
                        "Start Server",
                        Message::StartServer,
                    ))
                    .push(api_command_button(
                        &mut self.reload_server_state_button,
                        "Reload Status",
                        Message::ReloadServerState,
                    ))
                    .push(api_command_button(
                        &mut self.stop_server_button,
                        "Stop Server",
                        Message::StopServer,
                    )),
            )
            .push(Text::new(&self.err_message).color(Color::from_rgb(1.0, 0.0, 0.0)))
            .into()
    }
}

pub fn main() -> Result<()> {
    let window = window::Settings {
        size: (500, 200),
        ..Default::default()
    };
    MsmClient::run(Settings {
        window,
        ..Settings::default()
    })?;
    Ok(())
}
