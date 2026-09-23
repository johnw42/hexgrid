use eframe::egui;
use egui::Pos2;
use hexgrid::{
    Cartesian, Distance, HEX_HORIZONTAL_SPACING, HEX_VERTICAL_SPACING, HexCoord, HexCorner,
    HexCornerPos, HexEdge, HexEdgePos, HexGrid, HexGridSize, HexGroup, HexPerimeterIterator,
    HexPos, HexPosContainer as _, NearestCorner, NearestEdge,
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum InitSelection {
    None,
    Owned,
    Perimeter,
    PerimeterFromOrigin,
}

#[derive(Debug)]
struct GridContent<T> {
    init_param: T,
    is_active: bool,
}

impl<T> GridContent<T> {
    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }
}

type DemoGrid = HexGrid<GridContent<HexPos>, GridContent<HexEdgePos>, GridContent<HexCornerPos>>;

struct GridSelection {
    hexes: HexGroup<HexPos>,
    edges: HexGroup<HexEdgePos>,
    corners: HexGroup<HexCornerPos>,
}

impl GridSelection {
    fn new(grid: &DemoGrid) -> Self {
        let mut result = Self {
            hexes: HexGroup::new(),
            edges: HexGroup::new(),
            corners: HexGroup::new(),
        };
        for pos in grid.iter_hexes() {
            if grid.hex(pos).is_active {
                result.hexes.insert(pos);
            }
        }
        for edge in grid.iter_edges() {
            if grid.edge(edge).is_active {
                result.edges.insert(edge);
            }
        }
        for corner in grid.iter_corners() {
            if grid.corner(corner).is_active {
                result.corners.insert(corner);
            }
        }
        result
    }

    fn apply_to(&self, grid: &mut DemoGrid) {
        for pos in grid.iter_hexes() {
            grid.hex_mut(pos).is_active = self.hexes.contains(pos);
        }
        for edge in grid.iter_edges() {
            grid.edge_mut(edge).is_active = self.edges.contains(edge);
        }
        for corner in grid.iter_corners() {
            grid.corner_mut(corner).is_active = self.corners.contains(corner);
        }
    }
}

struct DemoApp {
    id: egui::Id,
    width: HexCoord,
    height: HexCoord,
    selection: InitSelection,
    grid: Option<DemoGrid>,
    grid_selection: Option<InitSelection>,
    perimeter_animation: Vec<HexEdgePos>,
}
const INIT_WIDTH: HexCoord = 7;
const INIT_HEIGHT: HexCoord = 9;

impl DemoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let id = egui::Id::new("demo_app");
        Self {
            id,
            width: INIT_WIDTH,
            height: INIT_HEIGHT,
            selection: InitSelection::None,
            grid: None,
            grid_selection: None,
            perimeter_animation: Vec::new(),
        }
    }

    fn create_grid(&mut self, ctx: &egui::Context) {
        self.perimeter_animation.clear();
        self.grid = HexGridSize::new(self.width, self.height).ok().map(|size| {
            DemoGrid::new(
                size,
                |pos| GridContent {
                    init_param: pos,
                    is_active: false,
                },
                |edge| GridContent {
                    init_param: edge,
                    is_active: self.selection == InitSelection::Owned
                        && size.contains_hex(edge.norm().0),
                },
                |corner| GridContent {
                    init_param: corner,
                    is_active: self.selection == InitSelection::Owned
                        && size.contains_hex(corner.norm().0),
                },
            )
        });

        if self.grid.is_some() {
            self.grid_selection = Some(self.selection);
            match self.selection {
                InitSelection::None | InitSelection::Owned => {}
                InitSelection::Perimeter => {
                    self.select_perimeter();
                }
                InitSelection::PerimeterFromOrigin => {
                    self.select_perimeter_from_origin(ctx);
                }
            }
        }
    }

    fn select_perimeter(&mut self) {
        if self.grid.is_some() {
            self.clear_edge_and_corner_selection();
            let grid = self.grid.as_mut().unwrap();
            let unselected_hexes = grid
                .iter_hexes()
                .filter(|&pos| !grid.hex(pos).is_active)
                .collect::<Vec<_>>();
            for edge_pos in HexPerimeterIterator::new(&unselected_hexes) {
                grid.edge_mut(edge_pos).is_active = true;
            }
        }
    }

    fn select_perimeter_from_origin(&mut self, ctx: &egui::Context) {
        self.clear_edge_and_corner_selection();
        ctx.animate_value_with_time(self.id, 0.0, 0.0);
        if let Some(grid) = self.grid.as_mut() {
            self.perimeter_animation =
                HexPerimeterIterator::new_from(HexPos::new(0, 0), grid).collect();
        }
    }

    fn clear_edge_and_corner_selection(&mut self) {
        if let Some(grid) = self.grid.as_mut() {
            for pos in grid.iter_hexes() {
                for edge in HexEdge::ALL {
                    grid.edge_mut(HexEdgePos::from((pos, edge))).is_active = false;
                }
                for corner in HexCorner::ALL {
                    grid.corner_mut(HexCornerPos::from((pos, corner))).is_active = false;
                }
            }
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
            ui.horizontal(|ui| {
                ui.label("Initial selection:");
                ui.radio_value(&mut self.selection, InitSelection::None, "None");
                ui.radio_value(&mut self.selection, InitSelection::Owned, "Owned");
                ui.radio_value(&mut self.selection, InitSelection::Perimeter, "Perimeter");
                ui.radio_value(
                    &mut self.selection,
                    InitSelection::PerimeterFromOrigin,
                    "Perimeter from Origin",
                );
            });

            if self
                .grid
                .as_ref()
                .is_none_or(|g| g.width() != self.width || g.height() != self.height)
                || self.grid_selection != Some(self.selection)
            {
                self.create_grid(ui.ctx());
            }

            if self.grid.is_some() {
                ui.separator();
                ui.add(HexView {
                    scale: 50.0,
                    app: self,
                });
            } else {
                ui.label("Invalid grid size");
            }

            if !self.perimeter_animation.is_empty() {
                let progress = ui.ctx().animate_value_with_time(
                    self.id,
                    (self.perimeter_animation.len() - 1) as f32,
                    1.0,
                );
                let edge_pos = self.perimeter_animation[progress as usize];
                let grid = self.grid.as_mut().unwrap();
                grid.edge_mut(edge_pos).is_active = true;
            }
        });
    }
}
struct HexView<'a> {
    scale: f32,
    app: &'a mut DemoApp,
}

fn reflect_vertical((x, y): Cartesian) -> Cartesian {
    (x, -y)
}

impl<'a> HexView<'a> {
    fn on_click(
        &mut self,
        hover_hex: HexPos,
        hover_edge: Option<HexEdgePos>,
        hover_corner: Option<HexCornerPos>,
    ) {
        let grid = self.app.grid.as_mut().unwrap();
        if let Some(hover_corner) = hover_corner {
            eprintln!(
                "Clicked corner {}; init_param: {}",
                hover_corner,
                grid.corner(hover_corner).init_param
            );
            // eprintln!(
            //     "corner_index: {:?}",
            //     grid.corner_index(corner_pos.pos(), corner_pos.corner())
            // );
            grid.corner_mut(hover_corner).toggle();
        } else if let Some(hover_edge) = hover_edge {
            eprintln!(
                "Clicked edge {}; init_param: {}",
                hover_edge,
                grid.edge(hover_edge).init_param
            );
            grid.edge_mut(hover_edge).toggle();
        } else {
            eprintln!(
                "Clicked hex {}; init_param: {}",
                hover_hex,
                grid.hex(hover_hex).init_param
            );
            grid.hex_mut(hover_hex).toggle();
            match self.app.selection {
                InitSelection::None | InitSelection::Owned | InitSelection::PerimeterFromOrigin => {
                }
                InitSelection::Perimeter => {
                    self.app.select_perimeter();
                }
            }
        }
    }

    fn on_secondary_click(&mut self, hover_hex: HexPos) {}
}

impl<'g> egui::Widget for HexView<'g> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
        let painter = ui.painter_at(rect);

        let HexView { scale, .. } = self;

        let grid = self.app.grid.as_ref().unwrap();

        let rect_offset = rect.center().to_vec2()
            - egui::vec2(
                (grid.width() - 1) as Distance * HEX_HORIZONTAL_SPACING,
                (1 - grid.height()) as Distance * HEX_VERTICAL_SPACING,
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
            .filter(|&pos| grid.has_hex(pos));

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

        for (i, pos) in grid.iter_hexes().enumerate() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
                    .into_iter()
                    .map(|corner| hex_to_widget(pos.corner_pos(corner)))
                    .collect(),
                if hover_hex == Some(pos) {
                    egui::Color32::RED
                } else if grid.hex(pos).is_active {
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
        for pos in grid.iter_hexes() {
            for edge in HexEdge::ALL {
                if grid.edge(HexEdgePos::from((pos, edge))).is_active {
                    paint_edge_line(pos, edge, 5.0, egui::Color32::WHITE);
                }
            }

            for corner in HexCorner::ALL {
                if grid.corner(HexCornerPos::from((pos, corner))).is_active {
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
                hover_corner = Some(HexCornerPos::from((hover_hex, corner)));
            }

            let NearestEdge { distance, edge } = hover_hex.nearest_edge(pos);
            if distance < 0.4 {
                hover_edge = Some(HexEdgePos::from((hover_hex, edge)));
            }
        }

        if let Some(hover_edge) = hover_edge {
            paint_edge_line(
                hover_edge.pos(),
                hover_edge.edge(),
                3.0,
                egui::Color32::GREEN,
            );
        }
        if let Some(hover_corner) = hover_corner {
            paint_corner_dot(
                hover_corner.pos(),
                hover_corner.corner(),
                5.0,
                egui::Color32::GREEN,
            );
        }

        if let Some(hover_hex) = hover_hex {
            if response.clicked() {
                self.on_click(hover_hex, hover_edge, hover_corner);
            }

            if response.secondary_clicked() {
                self.on_secondary_click(hover_hex);
            }
        }

        response
    }
}
