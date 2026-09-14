use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian,
    corner::HexCorner,
    edge::HexEdge,
    grid::HexGrid,
    pos::{HexPos, NearestCorner, NearestEdge},
};

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

fn reflect_vertical((x, y): Cartesian) -> Cartesian {
    (x, -y)
}

impl egui::Widget for HexView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let HexView { radius, .. } = self;

        let rect_offset = rect.center().to_vec2();
        let widget_to_hex = |pos: Pos2| -> Cartesian {
            let egui::Pos2 { x, y } = (pos - rect_offset) / radius;
            reflect_vertical((x, y))
        };
        let hex_to_widget = |(x, y): Cartesian| -> egui::Pos2 {
            (rect_offset + egui::Vec2::from(reflect_vertical((x, y))) * radius).to_pos2()
        };

        let latest_pos = ui
            .ctx()
            .input(|input| input.pointer.latest_pos())
            .map(widget_to_hex);
        let hover_hex = latest_pos.map(HexPos::from_center);

        for pos in HexPos::range(-3, -3, 3, 3) {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::all()
                    .into_iter()
                    .map(|corner| hex_to_widget(pos.corner_pos(corner)))
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else {
                    egui::Color32::BLACK
                },
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            ));

            painter.text(
                hex_to_widget(pos.center_pos()),
                egui::Align2::CENTER_CENTER,
                format!("{}", pos),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::WHITE,
            );
        }

        if let Some(hover_hex) = hover_hex {
            let pos = latest_pos.unwrap();

            let NearestEdge { distance, edge } = hover_hex.nearest_edge(pos);
            if distance < 0.5 {
                painter.line_segment(
                    [
                        hex_to_widget(hover_hex.corner_pos(edge.ends()[0])),
                        hex_to_widget(hover_hex.corner_pos(edge.ends()[1])),
                    ],
                    egui::Stroke::new(3.0, egui::Color32::GREEN),
                );
            }

            let NearestCorner {
                distance, point, ..
            } = hover_hex.nearest_corner(pos);
            if distance < 0.5 {
                painter.circle_filled(hex_to_widget(point), 5.0, egui::Color32::YELLOW);
            }
        }

        response
    }
}
