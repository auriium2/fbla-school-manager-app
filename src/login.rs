use async_trait::async_trait;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{Align2, Button, Color32, TextEdit, Ui, Window};
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use tokio::task::JoinHandle;
use crate::{ChannelComponent, ChannelLogic};

pub enum LoginCommand {
    TryLogin(String) //license
}

pub enum LoginResponse {
    LoginIs(bool)
}

#[derive(Debug)]
pub enum Internal {
    Success,
    Bad
}

pub struct LoginComponent {
    pub password: String,
    pub is_login_shown: bool,
    pub red_ticks: u8
}

#[async_trait]
impl ChannelLogic<LoginCommand, LoginResponse> for LoginLogic {
    async fn inner_update(&mut self, rx: Receiver<LoginCommand>, tx: Sender<LoginResponse>) {
        let mut should_notify_bad = false;
        let mut should_notify_good = false;

        for cmd in self.internal_receiver.try_iter() {
            match cmd {
                Internal::Success => {
                    for handle in &self.internals {
                        handle.abort()
                    }

                    self.internals.clear();
                    should_notify_good = true;
                    break; //we dont care if we need to notify bad
                }
                Internal::Bad => { should_notify_bad = true }
            }
        }

        for cmd in rx.try_iter() {
            match cmd {
                LoginCommand::TryLogin(license) => {
                    if !should_notify_good {
                        let handle = tokio::spawn( Self::spawn_check_query(self.client.clone(), self.internal_sender.clone(), license));
                        self.internals.push(handle);
                    }
                }
            }
        }

        if should_notify_good {
            tx.send(LoginResponse::LoginIs(true)).expect("sender failure");
        }

        if should_notify_bad {
            tx.send(LoginResponse::LoginIs(false)).expect("sender failure");
        }
    }
}

impl ChannelComponent<LoginCommand, LoginResponse> for LoginComponent {
    fn inner_view(&mut self, ui: &mut Ui, rx: Receiver<LoginResponse>, tx: Sender<LoginCommand>) {

    }
}

pub struct LoginLogic {
    pub internal_receiver: Receiver<Internal>,
    pub internal_sender: Sender<Internal>,
    pub internals: Vec<JoinHandle<()>>,

    pub client: Client
}



impl LoginLogic {
    async fn spawn_check_query(client: Client, sender: Sender<Internal>, license: String) {
        
        let res = client
            .get(format!("https://api.whop.com/api/v1/licenses/{0}", license))
            .header(AUTHORIZATION, "you dont get this one :(")
            .send()
            .await
            .expect("error sending!");

        let status = res.status();

        if status.is_success() {
            println!("yay");
            sender.send(Internal::Success).expect("send failed");
        } else {
            sender.send(Internal::Bad).expect("failed send");
        }

        let str = res.text().await.expect("bad");
        println!("ass: {0} capped with error {1}", str, status)
    }
}