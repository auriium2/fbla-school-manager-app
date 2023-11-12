use std::alloc::Layout;
use async_trait::async_trait;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{Align2, Button, CentralPanel, Context, FontId, Id, Label, Rect, RichText, Sense, Stroke, Ui, vec2, Vec2};
use eframe::{egui, Frame};
use eframe::egui::accesskit::TextAlign::Justify;
use eframe::emath::Align;
use crate::{ChannelComponent, ChannelLogic, ChannelPod};
use crate::login::{LoginCommand, LoginComponent, LoginLogic, LoginResponse};
use crate::students::{StudentsCommand, StudentsResponse};
use crate::students::comp::StudentsComponent;
use crate::students::logic::StudentsLogic;

pub struct CentralCommand {}
pub struct CentralResponse {}

enum Selected {
    STUDENTS,
    EVENTS,
    CHECK_IN
}

pub struct CentralComponent {
    pub students: ChannelPod<StudentsResponse, StudentsCommand, StudentsComponent>
}

pub struct CentralLogic {
    pub students: ChannelPod<StudentsCommand, StudentsResponse, StudentsLogic>
}

impl ChannelComponent<CentralCommand, CentralResponse> for CentralComponent {


    fn inner_view(&mut self, ui: &mut Ui, rx: Receiver<CentralResponse>, tx: Sender<CentralCommand>) {



        //never called :(

    }
}

#[async_trait]
impl ChannelLogic<CentralCommand, CentralResponse> for CentralLogic {
    async fn inner_update(&mut self, rx: Receiver<CentralCommand>, tx: Sender<CentralResponse>) {
        self.students.update().await;
    }
}

impl eframe::App for CentralComponent {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let text_color = ctx.style().visuals.text_color();
        let height = 30.0;
        let title = "UHS Student Event Tracker";

        ctx.set_debug_on_hover(true);


        CentralPanel::default()
            .frame(eframe::egui::Frame::none())
            .show(ctx, move |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();

                painter.rect(
                    rect.shrink(1.0),
                    10.0,
                    ctx.style().visuals.window_fill(),
                    Stroke::new(1.0, text_color),
                );

                painter.text(
                    rect.center_top() + vec2(0.0, height / 2.0),
                    Align2::CENTER_CENTER,
                    title,
                    FontId::proportional(height * 0.8),
                    text_color,
                );

                painter.line_segment(
                    [
                        rect.left_top() + vec2(2.0, height),
                        rect.right_top() + vec2(-2.0, height),
                    ],
                    Stroke::new(1.0, text_color),
                );

                let resp = ui.put(
                    Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                    Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
                );

                if resp.clicked() {
                    frame.close()
                }

                let title_bar_rect = {
                    let mut rect = rect;
                    rect.max.y = rect.min.y + height;
                    rect
                };

                let title_bar_response =
                    ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
                if title_bar_response.is_pointer_button_down_on() {
                    frame.drag_window();
                }

                let content_rect = {
                    let mut rect = rect;
                    rect.min.y = title_bar_rect.max.y;

                    rect
                }.shrink(5.0);

                let blob_rect = content_rect.shrink(25.0).translate(Vec2::new(-20.0,0.0));


                let mut total = ui.child_ui(content_rect, egui::Layout::left_to_right(Align::TOP));
                total.trace_location("total");

                let mut blob = ui.child_ui(blob_rect, egui::Layout::left_to_right(Align::TOP));
                blob.trace_location("blob");

                egui::TopBottomPanel::bottom("bottom").show_inside(&mut total, |uzi| {
                    uzi.horizontal_wrapped(|ui3| {
                        ui3.visuals_mut().button_frame = false;

                        egui::widgets::global_dark_light_mode_switch(ui3);

                        ui3.separator();

                        ui3.menu_button("Students", |ui| {

                        })

                        //bar contents should go here please
                    });
                });

                self.students.redraw(&mut blob);

            });
    }
}
