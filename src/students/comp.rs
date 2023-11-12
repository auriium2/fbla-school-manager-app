use crossbeam_channel::{Receiver, Sender};
use diesel::IntoSql;
use eframe::egui;
use eframe::egui::{Label, Ui};
use eframe::egui::panel::Side;
use egui_modal::{Icon, Modal};
use crate::ChannelComponent;
use crate::students::{StudentRecord, StudentsCommand, StudentsResponse};

//i love breaking threadsafety, but who cares, it only needs to work once
pub struct StudentsComponent {
    pub students_cache: Vec<StudentData> //these aren't accurate, should be sorted by points
}

pub struct StudentData {
    pub(crate) id: isize,
    pub(crate) name: String,
    pub(crate) grade: isize,
    pub(crate) points: isize
}

impl ChannelComponent<StudentsCommand, StudentsResponse> for StudentsComponent {
    fn inner_view(&mut self, ui: &mut Ui, rx: Receiver<StudentsResponse>, tx: Sender<StudentsCommand>) {

        let mut dialog = Modal::new(ui.ctx(), "my_modal");


        //populate data
        for rsp in rx.try_iter() {
            match rsp {
                //load data initially, only run this once
                StudentsResponse::GetAllSuccess(rsp) => {
                    for record in rsp {

                        let data = StudentData {
                            id: record.id,
                            name: record.name,
                            grade: record.grade,
                            points: record.points,
                        };

                        //update initial data
                        self.students_cache.push(data);


                    }
                }
                StudentsResponse::AddStudentSuccess(rec) => {

                    dialog.open_dialog(
                        Option::from("Success"),
                        Option::from("Added student successfully"),
                        Option::from(Icon::Success)
                    );



                    self.students_cache.push(StudentData {
                        id: rec.id,
                        name: rec.name,
                        grade: rec.grade,
                        points: rec.points,
                    });



                }
                StudentsResponse::PointsChangeEvent(a,b) => {

                    //gotta be the worst code i've ever written
                    for rec in self.students_cache.iter_mut() {
                        if rec.id == a {
                            rec.points = b;
                        }
                    }
                }
            }
        }

        dialog.show_dialog();

        egui::Grid::new("my_grid")
            .num_columns(4)
            .striped(true)
            .spacing([40.0, 40.0])
            .min_row_height(20.0)
            .min_col_width(210.0)
            .max_col_width(211.0)
            .show(ui, |ui4| {

                //how is this legal
                for val in self.students_cache.iter_mut() {
                    //id

                    ui4.add(Label::new(format!("User ID: {}", val.id)));

                    //name
                    let name_ref = &mut val.name;
                    let grade_ref = &mut val.grade;

                    //name
                    if ui4.add(egui::TextEdit::singleline(name_ref)).changed() {

                        //send the new name off
                        tx.send(StudentsCommand::Update(val.id, name_ref.clone(), *grade_ref)).expect("bad send");

                    }

                    //cum
                    if ui4.add(egui::Slider::new(grade_ref, 8..=12).suffix("th Grade")).changed() {
                        tx.send(StudentsCommand::Update(val.id, val.name.clone(), *grade_ref)).expect("bad send");
                    }

                    //points

                    ui4.add(Label::new(format!("User Points: {}", val.points)));


                    ui4.end_row();

                }
            });





        egui::SidePanel::right("rt")
            .show_inside(ui, |ui| {
                ui.label("sex")
            });


    }
}