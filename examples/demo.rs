use eframe::egui;
use hexgrid::{
    Cartesian, Distance, HEX_HORIZONTAL_SPACING, HEX_VERTICAL_SPACING, HexCoord, HexCorner,
    HexCornerPos, HexEdge, HexEdgePos, HexGrid, HexGridSize, HexGroup, HexPerimeterIterator,
    HexPos, HexPosContainer as _, HexRegionIterator, NearestCorner, NearestEdge,
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "hexgrid demo",
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

#[derive(Debug, Default, Clone)]
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
            grid.hex_mut(pos).is_active = false;
        }
        for edge in grid.iter_edges() {
            grid.edge_mut(edge).is_active = false;
        }
        for corner in grid.iter_corners() {
            grid.corner_mut(corner).is_active = false;
        }
        for pos in &self.hexes {
            if grid.has_hex(*pos) {
                grid.hex_mut(*pos).is_active = true;
            }
        }
        for edge in &self.edges {
            if grid.has_edge(*edge) {
                grid.edge_mut(*edge).is_active = true;
            }
        }
        for corner in &self.corners {
            if grid.has_corner(*corner) {
                grid.corner_mut(*corner).is_active = true;
            }
        }
    }

    fn rotate_around(self, center: HexPos, steps: HexCoord) -> Self {
        Self {
            hexes: self.hexes.rotate_around(center, steps),
            edges: self.edges.rotate_around(center, steps),
            corners: self.corners.rotate_around(center, steps),
        }
    }
}

type HoverState = Option<(HexPos, Option<HexEdge>, Option<HexCorner>)>;

struct DemoApp {
    id: egui::Id,
    width: HexCoord,
    height: HexCoord,
    init_selection: InitSelection,
    grid: Option<DemoGrid>,
    hover: HoverState,
    grid_selection: GridSelection,
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
            init_selection: InitSelection::None,
            grid: None,
            hover: None,
            grid_selection: GridSelection::default(),
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
                    is_active: self.init_selection == InitSelection::Owned
                        && size.contains_hex(edge.norm().pos()),
                },
                |corner| GridContent {
                    init_param: corner,
                    is_active: self.init_selection == InitSelection::Owned
                        && size.contains_hex(corner.norm().pos()),
                },
            )
        });

        if let Some(grid) = self.grid.as_ref() {
            self.grid_selection = GridSelection::new(grid);
            match self.init_selection {
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
        self.clear_edge_and_corner_selection();
        if let Some(grid) = self.grid.as_mut() {
            let unselected_hexes = grid
                .iter_hexes()
                .filter(|&pos| !grid.hex(pos).is_active)
                .collect::<Vec<_>>();
            self.grid_selection = GridSelection::default();
            for edge_pos in HexPerimeterIterator::new(&unselected_hexes) {
                grid.edge_mut(edge_pos).is_active = true;
                self.grid_selection.edges.insert(edge_pos);
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

    fn update_grid_selection(&mut self) {
        if let Some(grid) = self.grid.as_mut() {
            self.grid_selection = GridSelection::new(grid);
        } else {
            self.grid_selection = GridSelection::default();
        }
    }

    fn clear_selection(&mut self) {
        if let Some(grid) = self.grid.as_mut() {
            for pos in grid.iter_hexes() {
                grid.hex_mut(pos).is_active = false;
                for edge in HexEdge::ALL {
                    grid.edge_mut(HexEdgePos::from((pos, edge))).is_active = false;
                }
                for corner in HexCorner::ALL {
                    grid.corner_mut(HexCornerPos::from((pos, corner))).is_active = false;
                }
            }
        }
        self.update_grid_selection();
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
        self.update_grid_selection();
    }

    fn on_grid_clicked(&mut self) {
        if self.grid.is_none() {
            return;
        }
        match self.hover {
            Some((hover_hex, _, Some(hover_corner))) => {
                let grid = self.grid.as_mut().unwrap();
                let hover_corner = HexCornerPos::from((hover_hex, hover_corner));
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
            }
            Some((hover_hex, Some(hover_edge), None)) => {
                let grid = self.grid.as_mut().unwrap();
                let hover_edge = HexEdgePos::from((hover_hex, hover_edge));
                eprintln!(
                    "Clicked edge {}; init_param: {}",
                    hover_edge,
                    grid.edge(hover_edge).init_param
                );
                grid.edge_mut(hover_edge).toggle();
            }
            Some((hover_hex, None, None)) => {
                let grid = self.grid.as_mut().unwrap();
                eprintln!(
                    "Clicked hex {}; init_param: {}",
                    hover_hex,
                    grid.hex(hover_hex).init_param
                );
                grid.hex_mut(hover_hex).toggle();
                match self.init_selection {
                    InitSelection::None
                    | InitSelection::Owned
                    | InitSelection::PerimeterFromOrigin => {}
                    InitSelection::Perimeter => {
                        self.select_perimeter();
                    }
                }
            }
            _ => (),
        }
        self.update_grid_selection();
    }

    fn on_secondary_grid_clicked(&mut self) {
        if self.grid.is_some()
            && let Some((hover_hex, _, _)) = self.hover
        {
            self.grid_selection = self.grid_selection.clone().rotate_around(hover_hex, 1);
            self.grid_selection.apply_to(self.grid.as_mut().unwrap());
        }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let old_init_selection = self.init_selection;

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
                ui.radio_value(&mut self.init_selection, InitSelection::None, "None");
                ui.radio_value(&mut self.init_selection, InitSelection::Owned, "Owned");
                ui.radio_value(
                    &mut self.init_selection,
                    InitSelection::Perimeter,
                    "Perimeter",
                );
                ui.radio_value(
                    &mut self.init_selection,
                    InitSelection::PerimeterFromOrigin,
                    "Perimeter from Origin",
                );
            });

            if self
                .grid
                .as_ref()
                .is_none_or(|g| g.width() != self.width || g.height() != self.height)
                || self.init_selection != old_init_selection
            {
                self.create_grid(ui.ctx());
            }

            if self.grid.is_some() {
                ui.separator();
                let mut coordinate_translation = CoordinateTranslation::new(50.0);
                let hex_view = HexView {
                    grid: self.grid.as_ref().unwrap(),
                    hover: &mut self.hover,
                    coordinate_translation: &mut coordinate_translation,
                };
                let response = ui.add(hex_view);

                if response.clicked() {
                    self.on_grid_clicked();
                }
                if response.secondary_clicked() {
                    self.on_secondary_grid_clicked();
                }
                // if response.drag_started_by(egui::PointerButton::Primary) {
                //     self.drag_start = response.interact_pointer_pos();
                // }
                if response.dragged_by(egui::PointerButton::Primary) {
                    let painter = ui.painter_at(response.rect);
                    let selected_rect = egui::Rect::from_two_pos(
                        response.interact_pointer_pos().unwrap()
                            - response.total_drag_delta().unwrap(),
                        response.interact_pointer_pos().unwrap(),
                    );
                    painter.rect_stroke(
                        selected_rect,
                        2.0,
                        egui::Stroke::new(2.0, egui::Color32::from_white_alpha(0x80)),
                        egui::StrokeKind::Middle,
                    );
                    self.clear_selection();
                    let grid = self.grid.as_mut().unwrap();
                    for hex in HexRegionIterator::new_cartesian(
                        coordinate_translation.gui_to_hex(selected_rect.left_bottom()),
                        coordinate_translation.gui_to_hex(selected_rect.right_top()),
                    ) {
                        if grid.has_hex(hex) {
                            grid.hex_mut(hex).is_active = true;
                        }
                    }
                    self.update_grid_selection();
                }
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
                self.update_grid_selection();
            }
        });
    }
}

#[derive(Clone)]
struct CoordinateTranslation {
    scale: f32,
    rect_offset: egui::Vec2,
}

impl CoordinateTranslation {
    fn new(scale: f32) -> Self {
        Self {
            scale,
            rect_offset: egui::Vec2::default(),
        }
    }

    fn gui_to_hex(&self, pos: egui::Pos2) -> Cartesian {
        let egui::Pos2 { x, y } = (pos - self.rect_offset) / self.scale;
        Self::reflect_vertical((x, y))
    }

    fn hex_to_gui(&self, (x, y): Cartesian) -> egui::Pos2 {
        (self.rect_offset + egui::Vec2::from(Self::reflect_vertical((x, y))) * self.scale).to_pos2()
    }

    fn reflect_vertical((x, y): Cartesian) -> Cartesian {
        (x, -y)
    }
}

struct HexView<'a> {
    grid: &'a DemoGrid,
    hover: &'a mut HoverState,
    coordinate_translation: &'a mut CoordinateTranslation,
}

impl<'a> HexView<'a> {}

impl<'g> egui::Widget for HexView<'g> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        let Self {
            grid,
            coordinate_translation,
            hover,
        } = self;

        coordinate_translation.rect_offset = rect.center().to_vec2()
            - egui::vec2(
                (grid.width() - 1) as Distance * HEX_HORIZONTAL_SPACING,
                (1 - grid.height()) as Distance * HEX_VERTICAL_SPACING,
            ) * (coordinate_translation.scale / 2.0);

        let latest_pos = ui
            .ctx()
            .input(|input| input.pointer.latest_pos())
            .map(|pos| coordinate_translation.gui_to_hex(pos));
        *hover = latest_pos
            .map(HexPos::from_center)
            .filter(|&pos| grid.has_hex(pos))
            .map(|hover_hex| (hover_hex, None, None));

        let paint_edge_line = |pos: HexPos, edge: HexEdge, size: f32, color: egui::Color32| {
            let [corner1, corner2] = edge.ends();
            painter.line_segment(
                [
                    coordinate_translation.hex_to_gui(pos.corner_pos(corner1)),
                    coordinate_translation.hex_to_gui(pos.corner_pos(corner2)),
                ],
                egui::Stroke::new(size, color),
            );
        };

        let paint_corner_dot = |pos: HexPos, corner: HexCorner, size: f32, color: egui::Color32| {
            painter.circle_filled(
                coordinate_translation.hex_to_gui(pos.corner_pos(corner)),
                size,
                color,
            );
        };

        for (i, pos) in grid.iter_hexes().enumerate() {
            painter.add(egui::Shape::convex_polygon(
                HexCorner::ALL
                    .into_iter()
                    .map(|corner| coordinate_translation.hex_to_gui(pos.corner_pos(corner)))
                    .collect(),
                if hover.map(|(hex, _, _)| hex) == Some(pos) {
                    egui::Color32::RED
                } else if grid.hex(pos).is_active {
                    egui::Color32::BLUE
                } else {
                    egui::Color32::BLACK
                },
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            ));
            painter.text(
                coordinate_translation.hex_to_gui(pos.center_pos()),
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

        if let Some((hover_hex, hover_edge, hover_corner)) = hover {
            let pos = latest_pos.unwrap();

            let NearestCorner {
                distance, corner, ..
            } = hover_hex.nearest_corner(pos);
            if distance < 0.4 {
                *hover_corner = Some(corner);
            }

            let NearestEdge { distance, edge } = hover_hex.nearest_edge(pos);
            if distance < 0.4 {
                *hover_edge = Some(edge);
            }
        }

        if let Some((hover_hex, Some(hover_edge), _)) = *hover {
            paint_edge_line(hover_hex, hover_edge, 3.0, egui::Color32::GREEN);
        }
        if let Some((hover_hex, _, Some(hover_corner))) = *hover {
            paint_corner_dot(hover_hex, hover_corner, 5.0, egui::Color32::GREEN);
        }

        response
    }
}
