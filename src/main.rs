use std::env;
use std::thread::spawn;
use async_trait::async_trait;
use crossbeam_channel::{Receiver, Sender, unbounded};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use eframe::{App, NativeOptions};
use eframe::egui::{Ui, Vec2};
use crate::central::{CentralCommand, CentralComponent, CentralLogic, CentralResponse};
use crate::login::{LoginCommand, LoginComponent, LoginLogic, LoginResponse};
use crate::students::comp::{StudentData, StudentsComponent};
use crate::students::logic::StudentsLogic;
use crate::students::StudentsCommand;

mod central;
mod login;
mod students;
mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");



#[async_trait]
pub trait ChannelLogic<CMD,RES> {
    async fn inner_update(&mut self, rx: Receiver<CMD>, tx: Sender<RES>);
}

pub trait ChannelComponent<CMD,RES> {
    fn inner_view(&mut self, ui: &mut Ui, rx: Receiver<RES>, tx: Sender<CMD>);
}

pub struct ChannelPod<REC,RESP,C> {
    rx: Receiver<REC>,
    tx: Sender<RESP>,

    component: C,
}


fn new_pod<COMP,LOGIC,CMD,RES>(comp: COMP, logic: LOGIC) -> (ChannelPod<RES, CMD, COMP>, ChannelPod<CMD, RES, LOGIC>)
where
    COMP: ChannelComponent<CMD,RES>,
    LOGIC: ChannelLogic<CMD,RES>
{

    let (ta, ra) = unbounded();
    let (tb, rb) = unbounded();

    let pod_a = ChannelPod {
        rx: rb,
        tx: ta,
        component: comp
    };

    let pod_b = ChannelPod {
        rx: ra,
        tx: tb,
        component: logic
    };

    (pod_a, pod_b)
}

impl<A,B,C> ChannelPod<A, B, C> {
    fn access(&self) -> &C {
        &self.component
    }
}

impl<CMD,RES,OB> ChannelPod<RES,CMD,OB> where OB: ChannelComponent<CMD,RES> {
    fn redraw(&mut self, ui: &mut Ui) {
        self.component.inner_view(ui, self.rx.clone(), self.tx.clone())
    }
}


impl<CMD,RES,OB> ChannelPod<CMD, RES, OB> where OB: ChannelLogic<CMD,RES> {
    async fn update(&mut self) {
        self.component.inner_update(self.rx.clone(), self.tx.clone()).await;
    }
}

fn main() {

    //initialize db


    let con = SqliteConnection::establish("../local.sqlite").unwrap_or_else(|_| panic!("Error connecting to url"));

    let opts: NativeOptions = NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Option::Some(Vec2::new(1100.0, 700.0)),
        ..NativeOptions::default()
    };



    let student_logic = StudentsLogic {
        con,
    };



    let student_comp = StudentsComponent {
        students_cache: vec![
            StudentData {
                id: 0,
                name: "something sfw".to_string(),
                grade: 0,
                points: 0,
            },
            StudentData {
                id: 1,
                name: "something sfw 2".to_string(),
                grade: 0,
                points: 0,
            }
        ],
    };

    let (student_comp_pod, student_logic_pod) = new_pod(student_comp, student_logic);

    student_comp_pod.tx.send(StudentsCommand::GetAll).expect("bad send");


    let central = CentralComponent {
        students: student_comp_pod
    };


    let logic = CentralLogic {
        students: student_logic_pod
    };

    let (comp, mut logic) = new_pod(central, logic);


    spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    logic.update().await;
                }
            });
    });


    eframe::run_native("app", opts, Box::new(move |s| {Box::new(comp.component)}));


}



