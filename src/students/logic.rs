
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
                    let s = crate::schema::students::dsl::students.load::<DBRecord>(&mut self.con).expect("TODO: panic message");


                    let mut responses: Vec<StudentRecord> = vec![];



                    for record in s {
                        responses.push(StudentRecord {
                            id: record.id as isize,
                            name: record.name,
                            grade: record.grade as isize,
                            points: 0,
                        });
                    }

                }


                StudentsCommand::Update(_, _, _) => {}
                StudentsCommand::New(nam, grad) => {
                    let eqs = grade.eq(grad as i32);
                    let nam = name.eq(nam);
                    let lah = (eqs, nam);

                    let connec = &mut self.con;

                    let res = diesel::insert_into(students)
                        .values(lah)
                        .execute(&mut self.con);

                    let last_id = sql_function!("fn SELECT last_insert_rowid()");





                    diesel::select(last_id).execute(&mut self.con).expect("TODO: panic message");

                    if res.is_err() {
                        tx.send(StudentsResponse::AddStudentsFailure()).expect("bad send");
                    } else {
                        tx.send(StudentsResponse::AddStudentSuccess())
                    }


                }
            }
        }
    }
}

static ass: &str = "SELECT";
