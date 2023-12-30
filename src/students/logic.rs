
use async_trait::async_trait;
use crossbeam_channel::{Receiver, Sender};
use diesel::{SqliteConnection, Queryable, RunQueryDsl, Insertable, ExpressionMethods, sql_function};
use diesel::sql_types::{Int4, Integer, Nullable, Text, VarChar};
use serde_derive::{Deserialize, Serialize};
use sled::{IVec, Tree};
use crate::ChannelLogic;

use crate::schema::students::dsl::students;
use crate::schema::students::{grade, name, student_id};

use crate::students::{StudentRecord, StudentsCommand, StudentsResponse};

pub struct StudentsLogic {
    pub(crate) con: SqliteConnection

}

#[derive(Queryable)]
pub struct DBRecord {
    pub id: i32,
    pub name: String,
    pub grade: i32,

}

#[async_trait]
impl ChannelLogic<StudentsCommand, StudentsResponse> for StudentsLogic {
    async fn inner_update(&mut self, rx: Receiver<StudentsCommand>, tx: Sender<StudentsResponse>) {

        for cmd in rx.try_iter() {



            match cmd {
                StudentsCommand::GetAll => {


                }


                StudentsCommand::Update(_, _, _) => {}
                StudentsCommand::New(nam, grad) => {


                }
            }
        }
    }
}

static ass: &str = "SELECT";
