use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian, Distance, HEX_HEIGHT, HEX_WIDTH, HexCoord,
    corner::HexCorner,
    edge::HexEdge,
    grid::HexGrid,
    pos::{HexPos, NearestCorner, NearestEdge},
    validate_grid_size,
};
use std::panic::{self, AssertUnwindSafe, catch_unwind};

fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!(
            "Panic occurred at {:?}: {:?}",
            info.location(),
            info.payload_as_str()
        );
    }));
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    );
}

type DemoGrid = HexGrid<bool, bool, bool>;

struct DemoApp {
    width: HexCoord,
    height: HexCoord,
    grid: Option<DemoGrid>,
}

const INIT_WIDTH: HexCoord = 5;
const INIT_HEIGHT: HexCoord = 5;

impl DemoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            width: INIT_WIDTH,
            height: INIT_HEIGHT,
            grid: Some(DemoGrid::new_with_defaults(INIT_WIDTH, INIT_HEIGHT)),
        }
    }
}

impl eframe::App for DemoApp {
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
            ui.label(format!("Grid size: {} x {}", self.width, self.height));

            if self.grid.is_none()
                || self
                    .grid
                    .as_ref()
                    .is_none_or(|g| g.width() != self.width || g.height() != self.height)
            {
                self.grid = validate_grid_size(self.width, self.height)
                    .ok()
                    .map(|_| DemoGrid::new_with_defaults(self.width, self.height))
            }

            if let Some(grid) = &mut self.grid {
                ui.separator();
                ui.add(HexView { scale: 50.0, grid });
            } else {
                ui.label("Invalid grid size");
            }
        });
    }
}
struct HexView<'g> {
    scale: f32,
    grid: &'g mut DemoGrid,
}

fn reflect_vertical((x, y): Cartesian) -> Cartesian {
    (x, -y)
}

impl<'g> egui::Widget for HexView<'g> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
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

        let paint_edge_line = |pos: HexPos, edge: HexEdge, color: egui::Color32| {
            let [corner1, corner2] = edge.ends();
            painter.line_segment(
                [
                    hex_to_widget(pos.corner_pos(corner1)),
                    hex_to_widget(pos.corner_pos(corner2)),
                ],
                egui::Stroke::new(3.0, color),
            );
        };

        let paint_corner_dot = |pos: HexPos, corner: HexCorner, color: egui::Color32| {
            painter.circle_filled(hex_to_widget(pos.corner_pos(corner)), 5.0, color);
        };

        for (i, pos) in self.grid.hex_range().enumerate() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
                    .into_iter()
                    .map(|corner| hex_to_widget(pos.corner_pos(corner)))
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else if *self.grid.hex(pos) {
                    egui::Color32::BLUE
                } else {
                    egui::Color32::BLACK
                },
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            ));

            painter.text(
                hex_to_widget(pos.center_pos()),
                egui::Align2::CENTER_CENTER,
                format!("{}: {}", i, pos),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::WHITE,
            );

            for edge in [HexEdge::BottomLeft, HexEdge::Bottom, HexEdge::BottomRight] {
                eprintln!("getting edges for {}  {:?}", pos, edge);
                if let Ok(flag) = catch_unwind(AssertUnwindSafe(|| *self.grid.edge(pos, edge)))
                    && flag
                {
                    paint_edge_line(pos, edge, egui::Color32::WHITE);
                }
            }

            // for corner in HexCorner::ALL {
            //     if *self.grid.corner(pos, corner) {
            //         paint_corner_dot(pos, corner, egui::Color32::WHITE);
            //     }
            // }
        }

        for (pos, edge) in self.grid.edge_range() {
            match edge {
                HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => {}
                _ => {
                    paint_edge_line(pos, edge, egui::Color32::WHITE);
                }
            }
        }

        for (pos, corner) in self.grid.corner_range() {
            match corner {
                HexCorner::Right | HexCorner::TopRight | HexCorner::TopLeft => {}
                _ => {
                    paint_corner_dot(pos, corner, egui::Color32::WHITE);
                }
            }
        }

        let mut hover_edge = None;
        let mut hover_corner = None;
        if let Some(hover_hex) = hover_hex {
            let pos = latest_pos.unwrap();
            let NearestEdge { distance, edge } = hover_hex.nearest_edge(pos);
            if distance < 0.5 {
                hover_edge = Some((hover_hex, edge));
            }

            let NearestCorner {
                distance, corner, ..
            } = hover_hex.nearest_corner(pos);
            if distance < 0.5 {
                hover_corner = Some((hover_hex, corner));
            }
        }

        if let Some((hover_hex, hover_edge)) = hover_edge {
            paint_edge_line(hover_hex, hover_edge, egui::Color32::GREEN);
        }
        if let Some((hover_hex, hover_corner)) = hover_corner {
            paint_corner_dot(hover_hex, hover_corner, egui::Color32::GREEN);
        }

        if response.clicked() {
            let toggle = |flag: &mut bool| {
                *flag = !*flag;
            };

            if let Some(hover_hex) = hover_hex {
                toggle(self.grid.hex_mut(hover_hex));
            }
            if let Some((hover_hex, hover_edge)) = hover_edge {
                toggle(self.grid.edge_mut(hover_hex, hover_edge));
            }
            if let Some((hover_hex, hover_corner)) = hover_corner {
                toggle(self.grid.corner_mut(hover_hex, hover_corner));
            }
        }

        response
    }
}
