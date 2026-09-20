use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian, Distance, HEX_HEIGHT, HEX_WIDTH, HexCoord,
    corner::{HexCorner, HexPosWithCorner},
    edge::{HexEdge, HexPosWithEdge},
    grid::{HexGrid, HexGridSize},
    pos::{HexPos, NearestCorner, NearestEdge},
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    );
}

struct GridContent<T> {
    init_params: T,
    is_active: bool,
}

impl<T> GridContent<T> {
    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }
}

type DemoGrid =
    HexGrid<GridContent<HexPos>, GridContent<HexPosWithEdge>, GridContent<HexPosWithCorner>>;

fn new_demo_grid(size: &HexGridSize) -> DemoGrid {
    DemoGrid::new(
        size,
        |pos| GridContent {
            init_params: pos,
            is_active: false,
        },
        |edge| GridContent {
            init_params: edge,
            is_active: size.contains(edge.pos()),
        },
        |corner| GridContent {
            init_params: corner,
            is_active: size.contains(corner.pos()),
        },
    )
}

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
            grid: Some(new_demo_grid(
                &HexGridSize::new(INIT_WIDTH, INIT_HEIGHT).unwrap(),
            )),
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
                self.grid = HexGridSize::new(self.width, self.height)
                    .ok()
                    .map(|size| new_demo_grid(&size))
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

        let paint_edge_line = |pos: HexPos, edge: HexEdge, size: f32, color: egui::Color32| {
            let [corner1, corner2] = edge.ends();
            painter.line_segment(
                [
                    hex_to_widget(pos.corner_pos(corner1)),
                    hex_to_widget(pos.corner_pos(corner2)),
                ],
                egui::Stroke::new(size, color),
            );
        };

        let paint_corner_dot = |pos: HexPos, corner: HexCorner, size: f32, color: egui::Color32| {
            painter.circle_filled(hex_to_widget(pos.corner_pos(corner)), size, color);
        };

        // for (pos, corner) in self.grid.corner_range() {
        //     match corner {
        //         HexCorner::Right | HexCorner::TopRight | HexCorner::TopLeft => {}
        //         _ => {
        //             paint_corner_dot(pos, corner, egui::Color32::WHITE);
        //         }
        //     }
        // }

        for (i, pos) in self.grid.hex_range().enumerate() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
                    .into_iter()
                    .map(|corner| hex_to_widget(pos.corner_pos(corner)))
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else if self.grid.hex(pos).is_active {
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
        }
        for pos in self.grid.hex_range() {
            for edge in HexEdge::ALL {
                if self.grid.edge(pos, edge).is_active {
                    paint_edge_line(pos, edge, 5.0, egui::Color32::WHITE);
                }
            }

            for corner in HexCorner::ALL {
                if self.grid.corner(pos, corner).is_active {
                    paint_corner_dot(pos, corner, 7.0, egui::Color32::WHITE);
                }
            }
        }

        let mut hover_edge = None;
        let mut hover_corner = None;
        if let Some(hover_hex) = hover_hex {
            let pos = latest_pos.unwrap();

            let NearestCorner {
                distance, corner, ..
            } = hover_hex.nearest_corner(pos);
            if distance < 0.4 {
                hover_corner = Some((hover_hex, corner));
            }

            let NearestEdge { distance, edge } = hover_hex.nearest_edge(pos);
            if distance < 0.4 {
                hover_edge = Some((hover_hex, edge));
            }
        }

        if let Some((hover_hex, hover_edge)) = hover_edge {
            paint_edge_line(hover_hex, hover_edge, 3.0, egui::Color32::GREEN);
        }
        if let Some((hover_hex, hover_corner)) = hover_corner {
            paint_corner_dot(hover_hex, hover_corner, 5.0, egui::Color32::GREEN);
        }

        if response.clicked() {
            if let Some((hover_hex, hover_corner)) = hover_corner {
                eprintln!(
                    "Clicked corner {:?} of hex {:?}; index: {:?}, init_params: {:?}",
                    hover_corner,
                    hover_hex,
                    self.grid.corner_index(hover_hex, hover_corner),
                    self.grid.corner(hover_hex, hover_corner).init_params
                );
                self.grid.corner_mut(hover_hex, hover_corner).toggle();
            } else if let Some((hover_hex, hover_edge)) = hover_edge {
                eprintln!(
                    "Clicked edge {:?} of hex {:?}; index: {:?}, init_params: {:?}",
                    hover_edge,
                    hover_hex,
                    self.grid.edge_index(hover_hex, hover_edge),
                    self.grid.edge(hover_hex, hover_edge).init_params
                );
                self.grid.edge_mut(hover_hex, hover_edge).toggle();
            } else if let Some(hover_hex) = hover_hex {
                eprintln!(
                    "Clicked hex {:?}; init_params: {:?}",
                    hover_hex,
                    self.grid.hex(hover_hex).init_params
                );
                self.grid.hex_mut(hover_hex).toggle();
            }
        }

        response
    }
}
