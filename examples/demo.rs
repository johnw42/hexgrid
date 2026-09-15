use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian, HexCoord,
    corner::HexCorner,
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

struct MyEguiApp {
    left: HexCoord,
    top: HexCoord,
    right: HexCoord,
    bottom: HexCoord,
    grid: HexGrid<(), (), ()>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            left: 3,
            top: 3,
            right: 3,
            bottom: 3,
            grid: HexGrid::new(-3, -3, 3, 3),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Grid::new("my_grid").show(ui, |ui| {
                ui.label("Left");
                ui.add(egui::Slider::new(&mut self.left, 0..=10));
                ui.end_row();
                ui.label("Top");
                ui.add(egui::Slider::new(&mut self.top, 0..=10));
                ui.end_row();
                ui.label("Right");
                ui.add(egui::Slider::new(&mut self.right, 0..=10));
                ui.end_row();
                ui.label("Bottom");
                ui.add(egui::Slider::new(&mut self.bottom, 0..=10));
                ui.end_row();
            });

            // if ui.button("Regenerate").clicked() {
            //     self.grid = HexGrid::new(-self.left, -self.top, self.right, self.bottom);
            // }
            if self.top != -self.grid.top()
                || self.left != -self.grid.left()
                || self.right != self.grid.right()
                || self.bottom != self.grid.bottom()
            {
                self.grid = HexGrid::new(-self.left, -self.top, self.right, self.bottom);
            }

            ui.separator();

            ui.add(HexView {
                scale: 50.0,
                grid: &mut self.grid,
            });
        });
    }
}
struct HexView<'g> {
    pub scale: f32,
    pub grid: &'g mut HexGrid<(), (), ()>,
}

fn reflect_vertical((x, y): Cartesian) -> Cartesian {
    (x, -y)
}

impl<'g> egui::Widget for HexView<'g> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let HexView { scale, .. } = self;

        let rect_offset = rect.center().to_vec2();
        let widget_to_hex = |pos: Pos2| -> Cartesian {
            let egui::Pos2 { x, y } = (pos - rect_offset) / scale;
            reflect_vertical((x, y))
        };
        let hex_to_widget = |(x, y): Cartesian| -> egui::Pos2 {
            (rect_offset + egui::Vec2::from(reflect_vertical((x, y))) * scale).to_pos2()
        };

        let latest_pos = ui
            .ctx()
            .input(|input| input.pointer.latest_pos())
            .map(widget_to_hex);
        let hover_hex = latest_pos
            .map(HexPos::from_center)
            .filter(|&pos| self.grid.has_hex(pos));

        for pos in self.grid.range() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
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
