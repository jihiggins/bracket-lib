use crate::{
    consoles::{ConsoleFrontEnd, DrawBatch, DrawCommand, ScreenScaler},
    fonts::FontStore,
    TerminalScalingMode,
};
use bevy::{sprite::Mesh2dHandle, utils::HashMap};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point, Rect};
use parking_lot::Mutex;

pub struct BracketContext {
    pub(crate) fonts: Vec<FontStore>,
    pub(crate) terminals: Mutex<Vec<Box<dyn ConsoleFrontEnd>>>,
    pub(crate) current_layer: Mutex<usize>,
    pub(crate) color_palette: HashMap<String, RGBA>,
    pub fps: f64,
    pub frame_time_ms: f64,
    pub(crate) mesh_replacement: Vec<(Mesh2dHandle, Mesh2dHandle, bool)>,
    pub(crate) scaling_mode: TerminalScalingMode,
    command_buffers: Mutex<Vec<(usize, DrawBatch)>>,
    mouse_pixels: (f32, f32),
}

impl BracketContext {
    pub(crate) fn new(color_palette: HashMap<String, RGBA>) -> Self {
        Self {
            fonts: Vec::new(),
            terminals: Mutex::new(Vec::new()),
            current_layer: Mutex::new(0),
            color_palette,
            fps: 0.0,
            frame_time_ms: 0.0,
            mesh_replacement: Vec::new(),
            scaling_mode: TerminalScalingMode::Stretch,
            command_buffers: Mutex::new(Vec::new()),
            mouse_pixels: (0.0, 0.0),
        }
    }

    fn current_layer(&self) -> usize {
        *self.current_layer.lock()
    }

    pub fn get_char_size(&self) -> (usize, usize) {
        self.terminals.lock()[self.current_layer()].get_char_size()
    }

    pub fn get_pixel_size(&self) -> (f32, f32) {
        let mut pixel_size = (0.0, 0.0);
        self.terminals.lock().iter().for_each(|t| {
            let ts = t.get_pixel_size();
            pixel_size.0 = f32::max(pixel_size.0, ts.0);
            pixel_size.1 = f32::max(pixel_size.1, ts.1);
        });
        pixel_size
    }

    pub fn largest_font(&self) -> (f32, f32) {
        let mut result = (1.0, 1.0);
        self.fonts.iter().for_each(|fs| {
            result.0 = f32::max(result.0, fs.font_height_pixels.0);
            result.1 = f32::max(result.1, fs.font_height_pixels.1);
        });
        result
    }

    pub fn at(&self, x: usize, y: usize) -> usize {
        self.terminals.lock()[self.current_layer()].at(x, y)
    }

    pub fn try_at(&self, x: usize, y: usize) -> Option<usize> {
        self.terminals.lock()[self.current_layer()].try_at(x, y)
    }

    pub fn get_clipping(&self) -> Option<Rect> {
        self.terminals.lock()[self.current_layer()].get_clipping()
    }

    pub fn set_clipping(&self, clipping: Option<Rect>) {
        self.terminals.lock()[self.current_layer()].set_clipping(clipping);
    }

    pub fn set_layer(&self, layer: usize) {
        *self.current_layer.lock() = layer;
    }

    pub fn cls(&self) {
        self.terminals.lock()[self.current_layer()].cls();
    }

    pub fn cls_bg<C: Into<RGBA>>(&self, color: C) {
        self.terminals.lock()[self.current_layer()].cls_bg(color.into());
    }

    pub fn set<C: Into<RGBA>>(&self, x: usize, y: usize, fg: C, bg: C, glyph: u16) {
        self.terminals.lock()[self.current_layer()].set(x, y, fg.into(), bg.into(), glyph);
    }

    pub fn set_bg<C: Into<RGBA>>(&self, x: usize, y: usize, bg: C) {
        self.terminals.lock()[self.current_layer()].set_bg(x, y, bg.into());
    }

    pub fn print<S: ToString>(&self, x: usize, y: usize, text: S) {
        self.terminals.lock()[self.current_layer()].print(x, y, &text.to_string());
    }

    pub fn print_centered<S: ToString>(&self, y: usize, text: S) {
        self.terminals.lock()[self.current_layer()].print_centered(y, &text.to_string());
    }

    pub fn print_color_centered<S: ToString, C: Into<RGBA>>(
        &self,
        y: usize,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_centered(
            y,
            fg.into(),
            bg.into(),
            &text.to_string(),
        );
    }

    pub fn print_centered_at<S: ToString>(&self, x: usize, y: usize, text: S) {
        self.terminals.lock()[self.current_layer()].print_centered_at(x, y, &text.to_string());
    }

    pub fn print_color_centered_at<S: ToString, C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_centered_at(
            x,
            y,
            fg.into(),
            bg.into(),
            &text.to_string(),
        )
    }

    pub fn print_right<S: ToString>(&self, x: usize, y: usize, text: S) {
        self.terminals.lock()[self.current_layer()].print_right(x, y, &text.to_string());
    }

    pub fn print_color_right<S: ToString, C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_right(
            x,
            y,
            fg.into(),
            bg.into(),
            &text.to_string(),
        );
    }

    pub fn print_color<S: ToString, C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        text: S,
        foreground: C,
        background: C,
    ) {
        self.terminals.lock()[self.current_layer()].print_color(
            x,
            y,
            &text.to_string(),
            foreground.into(),
            background.into(),
        )
    }

    pub fn printer(
        &self,
        x: usize,
        y: usize,
        output: &str,
        align: crate::consoles::TextAlign,
        background: Option<RGBA>,
    ) {
        self.terminals.lock()[self.current_layer()].printer(self, x, y, output, align, background);
    }

    pub fn draw_box<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_box(
            x,
            y,
            width,
            height,
            fg.into(),
            bg.into(),
        );
    }

    pub fn draw_hollow_box<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_hollow_box(
            x,
            y,
            width,
            height,
            fg.into(),
            bg.into(),
        );
    }

    pub fn draw_box_double<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_box_double(
            x,
            y,
            width,
            height,
            fg.into(),
            bg.into(),
        );
    }

    pub fn draw_hollow_box_double<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_hollow_box_double(
            x,
            y,
            width,
            height,
            fg.into(),
            bg.into(),
        );
    }

    pub fn fill_region<C: Into<RGBA>>(&self, target: Rect, glyph: u16, fg: C, bg: C) {
        self.terminals.lock()[self.current_layer()].fill_region(
            target,
            glyph,
            fg.into(),
            bg.into(),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_horizontal<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        width: usize,
        n: usize,
        max: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_bar_horizontal(
            x,
            y,
            width,
            n,
            max,
            fg.into(),
            bg.into(),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_vertical<C: Into<RGBA>>(
        &self,
        x: usize,
        y: usize,
        height: usize,
        n: usize,
        max: usize,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_bar_vertical(
            x,
            y,
            height,
            n,
            max,
            fg.into(),
            bg.into(),
        );
    }

    pub fn set_all_fg_alpha(&self, alpha: f32) {
        self.terminals.lock()[self.current_layer()].set_all_bg_alpha(alpha);
    }

    pub fn set_all_bg_alpha(&self, alpha: f32) {
        self.terminals.lock()[self.current_layer()].set_all_bg_alpha(alpha);
    }

    pub fn set_all_alpha(&self, fg: f32, bg: f32) {
        self.terminals.lock()[self.current_layer()].set_all_alpha(fg, bg);
    }

    pub fn get_named_color(&self, color: &str) -> Option<&RGBA> {
        self.color_palette.get(color)
    }

    pub(crate) fn resize_terminals(&mut self, scaler: &ScreenScaler) {
        let available_size = scaler.available_size();
        let mut lock = self.terminals.lock();
        lock.iter_mut().for_each(|t| t.resize(&available_size));
    }

    pub fn new_draw_batch(&self) -> DrawBatch {
        DrawBatch::new()
    }

    pub fn submit_batch(&self, z_order: usize, mut batch: DrawBatch) {
        if batch.needs_sort {
            batch.batch.sort_by(|a, b| a.0.cmp(&b.0));
        }
        self.command_buffers.lock().push((z_order, batch));
    }

    pub fn render_all_batches(&mut self) {
        let mut batches = self.command_buffers.lock();
        batches.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        batches.iter().for_each(|(_, batch)| {
            batch.batch.iter().for_each(|(_, cmd)| match cmd {
                DrawCommand::ClearScreen => self.cls(),
                DrawCommand::ClearToColor { color } => self.cls_bg(*color),
                DrawCommand::SetTarget { console } => self.set_layer(*console),
                DrawCommand::Set { pos, color, glyph } => {
                    self.set(pos.x as usize, pos.y as usize, color.fg, color.bg, *glyph)
                }
                DrawCommand::SetBackground { pos, bg } => {
                    self.set_bg(pos.x as usize, pos.y as usize, *bg)
                }
                DrawCommand::Print { pos, text } => {
                    self.print(pos.x as usize, pos.y as usize, &text)
                }
                DrawCommand::PrintColor { pos, text, color } => {
                    self.print_color(pos.x as usize, pos.y as usize, &text, color.fg, color.bg)
                }
                DrawCommand::PrintCentered { y, text } => self.print_centered(*y as usize, &text),
                DrawCommand::PrintColorCentered { y, text, color } => {
                    self.print_color_centered(*y as usize, color.fg, color.bg, &text)
                }
                DrawCommand::PrintCenteredAt { pos, text } => {
                    self.print_centered_at(pos.x as usize, pos.y as usize, &text)
                }
                DrawCommand::PrintColorCenteredAt { pos, text, color } => self
                    .print_color_centered_at(
                        pos.x as usize,
                        pos.y as usize,
                        color.fg,
                        color.bg,
                        &text,
                    ),
                DrawCommand::PrintRight { pos, text } => {
                    self.print_right(pos.x as usize, pos.y as usize, text)
                }
                DrawCommand::PrintColorRight { pos, text, color } => {
                    self.print_color_right(pos.x as usize, pos.y as usize, color.fg, color.bg, text)
                }
                DrawCommand::Printer {
                    pos,
                    text,
                    align,
                    background,
                } => self.printer(pos.x as usize, pos.y as usize, text, *align, *background),
                DrawCommand::Box { pos, color } => self.draw_box(
                    pos.x1 as usize,
                    pos.y1 as usize,
                    pos.width() as usize,
                    pos.height() as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::HollowBox { pos, color } => self.draw_hollow_box(
                    pos.x1 as usize,
                    pos.y1 as usize,
                    pos.width() as usize,
                    pos.height() as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::DoubleBox { pos, color } => self.draw_box_double(
                    pos.x1 as usize,
                    pos.y1 as usize,
                    pos.width() as usize,
                    pos.height() as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::HollowDoubleBox { pos, color } => self.draw_hollow_box_double(
                    pos.x1 as usize,
                    pos.y1 as usize,
                    pos.width() as usize,
                    pos.height() as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::FillRegion { pos, color, glyph } => {
                    self.fill_region::<RGBA>(*pos, *glyph, color.fg, color.bg)
                }
                DrawCommand::BarHorizontal {
                    pos,
                    width,
                    n,
                    max,
                    color,
                } => self.draw_bar_horizontal(
                    pos.x as usize,
                    pos.y as usize,
                    *width as usize,
                    *n as usize,
                    *max as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::BarVertical {
                    pos,
                    height,
                    n,
                    max,
                    color,
                } => self.draw_bar_vertical(
                    pos.x as usize,
                    pos.y as usize,
                    *height as usize,
                    *n as usize,
                    *max as usize,
                    color.fg,
                    color.bg,
                ),
                DrawCommand::SetClipping { clip } => self.set_clipping(*clip),
                DrawCommand::SetFgAlpha { alpha } => self.set_all_fg_alpha(*alpha),
                DrawCommand::SetBgAlpha { alpha } => self.set_all_fg_alpha(*alpha),
                DrawCommand::SetAllAlpha { fg, bg } => self.set_all_alpha(*fg, *bg),
            })
        });

        batches.clear();
    }

    pub(crate) fn set_mouse_pixel_position(&mut self, pos: (f32, f32), scaler: &ScreenScaler) {
        self.mouse_pixels = pos;
        self.terminals
            .lock()
            .iter_mut()
            .for_each(|t| t.set_mouse_position(pos, scaler));
    }

    pub fn get_mouse_position_in_pixels(&self) -> (f32, f32) {
        self.mouse_pixels
    }

    pub fn get_mouse_position_for_current_layer(&self) -> Point {
        self.terminals.lock()[self.current_layer()].get_mouse_position_for_current_layer()
    }
}