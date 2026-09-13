use eframe::egui;
use egui::Pos2;
use hexgrid::{corner::HexCorner, edge::HexEdge, grid::HexGrid, pos::HexPos};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

struct MyEguiApp {
    my_boolean: bool,
    my_string: String,
    my_f32: f32,
    my_enum: Enum,
    my_image: egui::TextureId,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            my_boolean: false,
            my_string: String::new(),
            my_f32: 0.0,
            my_enum: Enum::First,
            my_image: egui::TextureId::default(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("This is a label");
            ui.hyperlink("https://github.com/emilk/egui");
            ui.text_edit_singleline(&mut self.my_string);
            if ui.button("Click me").clicked() {
                println!("Clicked!");
            }
            ui.add(egui::Slider::new(&mut self.my_f32, 0.0..=100.0));
            ui.add(egui::DragValue::new(&mut self.my_f32));

            ui.checkbox(&mut self.my_boolean, "Checkbox");

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.my_enum, Enum::First, "First");
                ui.radio_value(&mut self.my_enum, Enum::Second, "Second");
                ui.radio_value(&mut self.my_enum, Enum::Third, "Third");
            });

            ui.separator();

            //ui.image((self.my_image, egui::Vec2::new(640.0, 480.0)));

            ui.add(HexView {
                radius: 50.0,
                grid: HexGrid::new(3, 3),
            });

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });
        });
    }
}
struct HexView {
    pub radius: f32,
    pub grid: HexGrid<()>,
}

fn reflect_vertical((x, y): (f32, f32)) -> (f32, f32) {
    (x, -y)
}

impl egui::Widget for HexView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let HexView { radius, .. } = self;

        let rect_offset = rect.center() - egui::pos2(0.0, 0.0);

        let hover_hex = ui
            .ctx()
            .input(|input| input.pointer.latest_pos())
            .map(|pos| {
                let pos = (pos - rect_offset) / radius;
                HexPos::from_center(reflect_vertical((pos.x, pos.y)))
            });

        for (i, pos) in HexPos::range(-3, -3, 3, 2).enumerate() {
            let hex_offset =
                rect_offset + egui::Vec2::from(reflect_vertical(pos.center())) * radius;

            painter.add(egui::Shape::convex_polygon(
                HexCorner::all()
                    .into_iter()
                    .map(|corner| {
                        let (x, y) = corner.offset_from_center(radius);
                        egui::pos2(x, y) + hex_offset
                    })
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else {
                    egui::Color32::BLACK
                },
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            ));

            painter.text(
                egui::pos2(0.0, 0.0) + hex_offset,
                egui::Align2::CENTER_CENTER,
                format!("{}: {}", i, pos),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::WHITE,
            );

            // for [d1, d2] in HexEdge::all().into_iter().map(|edge| edge.ends()) {
            //     painter.line_segment(
            //         [
            //             egui::Pos2::from(d1.offset_from_center(radius)) + hex_offset,
            //             egui::Pos2::from(d2.offset_from_center(radius)) + hex_offset,
            //         ],
            //         egui::Stroke::new(
            //             1.0,
            //             if hover_hex == Some(pos) {
            //             } else {
            //                 egui::Color32::WHITE
            //             },
            //         ),
            //     );
            // }
        }

        response
    }
}
