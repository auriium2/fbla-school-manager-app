pub mod comp;
pub mod logic;

use async_trait::async_trait;
use crossbeam_channel::{Receiver, Sender};

use sled::{Db, IVec, Tree};
use crate::ChannelLogic;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct StudentRecord {
    pub id: isize,
    pub(crate) name: String,
    pub(crate) grade: isize,
    pub(crate) points: isize
}

pub enum StudentsCommand {
    GetAll,
    Update(isize, String, isize),
    New(String, isize) //name, grade, 0ed points
}

pub enum StudentsResponse {
    GetAllSuccess(Vec<StudentRecord>),


    AddStudentSuccess(StudentRecord),
    AddStudentsFailure(),

    PointsChangeEvent(isize, isize) //id, new
}
