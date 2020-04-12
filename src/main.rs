#![windows_subsystem = "windows"]

use crate::server::{post_request_to_server, PostRequestType, ServerState};
use iced::{
    button, executor, window, Align, Application, Button, Color, Column, Command, Element,
    HorizontalAlignment, Length, Row, Settings, Text,
};
use std::time::Duration;

mod config;
mod server;

/// メイン画面
#[derive(Debug, Eq, PartialEq, Default)]
struct ServerManager {
    /// 現在のサーバーの状態
    current_state: ServerState,
    /// エラー発生時のエラー内容
    err_message: String,
    /// サーバの状態確認を行った回数
    state_check_count: u16,
    /// サーバの起動ボタン
    start_server_button: button::State,
    /// サーバの状態を確認するボタン
    reload_server_state_button: button::State,
    /// サーバの停止ボタン
    stop_server_button: button::State,
}

/// イベントメッセージ
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    /// 開始ボタンが押された
    StartServer,
    /// リロードボタンが押された
    ReloadServerStatus,
    /// 停止ボタンが押された
    StopServer,
    /// サーバ状態の表示を変更する
    UpdateServerState(Result<ServerState, String>),
}

impl ServerManager {
    /// ロード回数カウントを生成する
    fn generate_state_count_str(&self) -> String {
        let mut out = String::new();
        for _count in 0..self.state_check_count {
            out += ".";
        }
        out
    }
}

impl Application for ServerManager {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            ServerManager::default(),
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
            Message::StartServer => {
                self.current_state = ServerState::Connecting;
                Command::perform(
                    post_request_to_server(PostRequestType::StartServer, None),
                    Message::UpdateServerState,
                )
            }
            Message::ReloadServerStatus => {
                self.current_state = ServerState::Connecting;
                Command::perform(
                    post_request_to_server(PostRequestType::GetServerStatus, None),
                    Message::UpdateServerState,
                )
            }
            Message::StopServer => {
                self.current_state = ServerState::Connecting;
                Command::perform(
                    post_request_to_server(PostRequestType::StopServer, None),
                    Message::UpdateServerState,
                )
            }
            Message::UpdateServerState(update) => match update {
                Ok(state) => {
                    self.current_state = state;
                    match state {
                        ServerState::Pending | ServerState::Stopping => {
                            self.state_check_count += 1;
                            Command::perform(
                                post_request_to_server(
                                    PostRequestType::GetServerStatus,
                                    Some(Duration::from_millis(1500)),
                                ),
                                Message::UpdateServerState,
                            )
                        }
                        _ => {
                            self.state_check_count = 0;
                            Command::none()
                        }
                    }
                }
                Err(err) => {
                    self.err_message = "Err: ".to_string() + &err;
                    Command::none()
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let row_button = |state, label, message| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .width(Length::Fill)
            .on_press(message)
        };

        Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Text::new(&self.current_state.to_string())
                    .color(self.current_state.color())
                    .size(30),
            )
            .push(Text::new(self.generate_state_count_str()).size(30))
            .push(
                Row::new()
                    .spacing(8)
                    .push(row_button(
                        &mut self.start_server_button,
                        "Start Server",
                        Message::StartServer,
                    ))
                    .push(row_button(
                        &mut self.reload_server_state_button,
                        "Reload Status",
                        Message::ReloadServerStatus,
                    ))
                    .push(row_button(
                        &mut self.stop_server_button,
                        "Stop Server",
                        Message::StopServer,
                    )),
            )
            .push(Text::new(&self.err_message).color(Color::from_rgb(1.0, 0.0, 0.0)))
            .into()
    }
}

pub fn main() {
    let window = window::Settings {
        size: (500, 200),
        resizable: true,
        decorations: true,
    };
    ServerManager::run(Settings {
        window,
        ..Settings::default()
    })
}
