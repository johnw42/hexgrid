use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian, Distance, HEX_HEIGHT, HEX_WIDTH, HexCoord,
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
    width: HexCoord,
    height: HexCoord,
    grid: HexGrid<(), (), ()>,
}

const INIT_WIDTH: HexCoord = 3;
const INIT_HEIGHT: HexCoord = 3;

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            width: INIT_WIDTH,
            height: INIT_HEIGHT,
            grid: HexGrid::new(INIT_WIDTH, INIT_HEIGHT),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Grid::new("my_grid").show(ui, |ui| {
                ui.label("Width");
                ui.add(egui::Slider::new(&mut self.width, 0..=10));
                ui.end_row();
                ui.label("Height");
                ui.add(egui::Slider::new(&mut self.height, 0..=10));
                ui.end_row();
            });
            ui.label(format!(
                "Grid size: {} x {}",
                self.grid.width(),
                self.grid.height()
            ));

            // if ui.button("Regenerate").clicked() {
            //     self.grid = HexGrid::new(-self.left, -self.bottom, self.right, self.top);
            // }
            if self.width != self.grid.width() || self.height != self.grid.height() {
                self.grid = HexGrid::new(self.width, self.height);
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

        let rect_offset = rect.center().to_vec2()
            - egui::vec2(
                (self.grid.width() - 1) as Distance * HEX_WIDTH,
                (1 - self.grid.height()) as Distance * HEX_HEIGHT,
            ) * (scale / 2.0);
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

        for (i, pos) in self.grid.range().enumerate() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
                    .into_iter()
                    .map(|corner| hex_to_widget(pos.corner_pos(corner)))
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else if pos == HexPos::new(0, 0) {
                    egui::Color32::BLUE
                } else {
                    egui::Color32::BLACK
                },
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            ));

            painter.text(
                hex_to_widget(pos.center_pos()),
                egui::Align2::CENTER_CENTER,
                format!("{}({}): {}", i, "", pos),
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
