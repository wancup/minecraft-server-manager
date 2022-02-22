#![windows_subsystem = "windows"]

use crate::server::{post_request_to_server, PostRequestType, ResponsePayload, ServerState};
use clipboard::{ClipboardContext, ClipboardProvider};
use iced::{
    button, executor, window, Align, Application, Button, Color, Column, Command, Element,
    HorizontalAlignment, Length, Row, Settings, Text,
};
use std::time::Duration;

mod config;
mod server;

/// メイン画面
#[derive(Debug, Default)]
struct ServerManager {
    /// 現在のサーバーの情報
    server_info: ResponsePayload,
    /// エラー発生時のエラー内容
    err_message: String,
    /// サーバの状態確認を行った回数
    state_check_count: u16,
    /// サーバIPをコピーするボタン
    copy_address_button: button::State,
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
    /// アドレスコピーボタンが押された
    CopyServerAddress,
    /// リロードボタンが押された
    ReloadServerStatus,
    /// 停止ボタンが押された
    StopServer,
    /// サーバ状態の表示を変更する
    UpdateServerState(Result<ResponsePayload, String>),
    /// サーバIPのTextInputが変更された
    AddressInputIsChanged(String),
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

    /// サーバの状態確認後に行うコマンドを取得する
    fn get_next_command_after_update_state(
        &mut self,
        server_info: ResponsePayload,
    ) -> Command<Message> {
        let next_command = match server_info.server_state {
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
        };
        self.server_info = server_info;
        next_command
    }

    /// サーバのIPアドレスをクリップボードにコピーする
    fn copy_address_to_clipboard(&mut self, address: String) {
        // TODO: fix unwrap
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let copy_result = ctx.set_contents(address);
        match copy_result {
            Ok(_) => (),
            Err(err) => self.err_message = err.to_string(),
        }
    }

    /// サーバの起動コマンドを実行する
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

    /// サーバの停止コマンドを実行する
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
            Message::StartServer => self.perform_start_server_command(),
            Message::CopyServerAddress => {
                match self.server_info.ip_address.clone() {
                    Some(address) => self.copy_address_to_clipboard(address),
                    None => (), // NOP
                };
                Command::none()
            }
            Message::ReloadServerStatus => {
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

        let checking_counter_str = self.generate_state_count_str();
        Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Text::new(&self.server_info.server_state.to_string())
                    .color(self.server_info.server_state.color())
                    .size(30),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .push(Text::new("Server Address:"))
                    .spacing(10)
                    .push(Text::new(
                        &self.server_info.ip_address.clone().unwrap_or(String::new()),
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
