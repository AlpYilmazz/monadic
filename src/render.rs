
use std::io::Write;
use std::{self, io::Stdout};
use std::fmt::Display;
use std::fs;
use std::path::Path;


use bevy::prelude::{Query, Component, With, Res, ResMut, Plugin, StartupStage, CoreStage, Commands};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal;


#[derive(Component, Clone)]
pub struct Coord(pub i32, pub i32);

#[derive(Component, Clone)]
pub struct Point(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct Rect{pub h: u32, pub w: u32}

impl Rect {
    #[inline]
    pub fn local_index(&self, i: u32, j: u32) -> usize {
        (self.w * i + j) as usize
    }

    #[inline]
    pub fn point_index(&self, Point(i, j): Point) -> usize {
        self.local_index(i, j)
    }
}

#[derive(Component)]
pub struct Sprite {
    pub shape: Rect,
    pub texture: Vec<char>,
}

impl Sprite {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let s: String = fs::read_to_string(path).expect("file could not be read");
        Self::new(&s)
    }

    pub fn new(base: &str) -> Self {
        if base.chars().all(|c| c.is_whitespace()) {
            return Self {
                shape: Rect { h: 0, w: 0 },
                texture: vec![],
            }
        }

        let mut shape = Rect {h: 0, w: 0};
        let mut texture = Vec::with_capacity(base.len());

        let (mut min_i, mut max_i, mut min_j, mut max_j)
                        = (usize::MAX, 0, usize::MAX, 0);
        for (i, line) in base.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if !c.is_whitespace() {
                    min_i = min_i.min(i);
                    max_i = max_i.max(i);
                    min_j = min_j.min(j);
                    max_j = max_j.max(j);
                }
            }
        }
        shape.h = (max_i - min_i + 1) as u32;
        shape.w = (max_j - min_j + 1) as u32;

        // println!("{min_i}, {max_i}, {min_j}, {max_j}");

        for line in base.lines().skip(min_i).take(max_i - min_i + 1) {
            let a = &line[min_j..(max_j+1).min(line.len())];
            let mut v: Vec<char> = a.chars().collect();
            let r = (max_j+1).saturating_sub(line.len());
            // println!("{:?}\n{:?}\n{:?}", a, v, r);
            for _ in 0..r {
                v.push(' ');
            }
            texture.append(&mut v);
        }

        Self {
            shape,
            texture
        }
    }

    pub fn get(&self, point: Point) -> Option<char> {
        self.texture.get(self.shape.point_index(point)).cloned()
    }

    pub fn get_ext(&self, point: Point) -> char {
        self.get(point).unwrap_or(' ')
    }
}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})\n", &self.shape.h, &self.shape.w)?;s
        write!(f, "+{}+\n", "-".repeat(self.shape.w as usize))?;
        for i in 0..self.shape.h {
            // TODO: handle `Result` return type
            write!(f, "|{}|\n", &self.texture[(i*self.shape.w) as usize .. ((i+1)*self.shape.w) as usize]
                                        .iter().collect::<String>())?;
        }
        write!(f, "+{}+\n", "-".repeat(self.shape.w as usize))?;
    
        Ok(())
    }
}

impl From<&DisplayBuffer> for Sprite {
    fn from(display: &DisplayBuffer) -> Self {
        Sprite { shape: display.shape.clone(), texture: display.screen_buffer.clone() }
    }
}

impl From<DisplayBuffer> for Sprite {
    fn from(display: DisplayBuffer) -> Self {
        Sprite { shape: display.shape, texture: display.screen_buffer }
    }
}

pub struct DisplayConfig {
    pub h: u32,
    pub w: u32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            h: 50,
            w: 100,
        }
    }
}

pub struct DisplayBuffer {
    pub shape: Rect,
    pub screen_buffer: Vec<char>,
}

impl DisplayBuffer {
    pub fn init_with(config: &DisplayConfig) -> Self {
        Self::init(config.h, config.w)
    }

    pub fn init(h: u32, w: u32) -> Self {
        Self {
            shape: Rect{h, w},
            screen_buffer: vec![' '; (h*w) as usize],
        }
    }

    pub fn clear(&mut self) {
        for px in self.screen_buffer.iter_mut() {
            *px = ' ';
        }
    }

    pub fn set_single(&mut self, point: Point, c: char) {
        let i = self.shape.point_index(point);
        self.screen_buffer[i] = c;
    }

    pub fn render(&mut self, coord: Coord, sprite: &Sprite) {
        let Coord(pos_h, pos_w) = coord;
        let Rect{h: sprite_h, w: sprite_w} = sprite.shape;
        let Rect{h: display_h, w: display_w} = self.shape;
        let (sprite_h, sprite_w, display_h, display_w) = (
            sprite_h as i32, 
            sprite_w as i32, 
            display_h as i32, 
            display_w as i32
        );
        let mfrom   = ((-pos_h).max(0), (-pos_w).max(0));
        let mto = (sprite_h.min(display_h - pos_h),
                            sprite_w.min(display_w - pos_w));
        let mdif    = ((mto.0 - mfrom.0) as u32, (mto.1 - mfrom.1) as u32);
        let posc     = ((pos_h.clamp(0, display_h)) as u32, (pos_w.clamp(0, display_w)) as u32);

        for i in 0..mdif.0 {
            for j in 0..mdif.1 {
                let display_f = self.shape.local_index(posc.0 + i, posc.1 + j);
                self.screen_buffer[display_f] = sprite.get_ext(Point(mfrom.0 as u32 + i, mfrom.1 as u32 + j));
            }
        }
    }

    pub fn get_buffer(&self) -> String {
        format!("{}", self)
    }
}

impl Display for DisplayBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Sprite::from(self).fmt(f)
    }
}

#[derive(Component)]
pub struct Visible;

pub fn set_default_display_config(
    mut commands: Commands,
    display_config: Option<Res<DisplayConfig>>,
) {
    match display_config {
        Some(_) => {},
        None => commands.insert_resource(
            DisplayConfig::default()
        ),
    }
}

pub fn init_display(
    mut commands: Commands,
    display_config: Res<DisplayConfig>,
) {
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, 
        terminal::Clear(terminal::ClearType::All),
        terminal::Clear(terminal::ClearType::Purge)
    ).expect("Terminal Purge failed on startup");
    
    commands.insert_resource(stdout);
    commands.insert_resource(DisplayBuffer::init_with(&display_config));
}

pub fn render_objects(
    mut display_buffer: ResMut<DisplayBuffer>,
    objects: Query<(&Sprite, &Coord), With<Visible>>,
) {
    for (sprite, position) in objects.iter() {
        display_buffer.render(position.clone(), sprite);
    }
}

pub fn swap_buffers(
    mut stdout: ResMut<Stdout>,
    display_buffer: Res<DisplayBuffer>,
) {
    let buffer = display_buffer.get_buffer();

    let result = queue!(stdout, 
        terminal::Clear(terminal::ClearType::All),
        terminal::Clear(terminal::ClearType::Purge),
        crossterm::cursor::MoveTo(0, 0),
        Print(buffer)
    );
    
    let flush_result = stdout.flush();
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
       app
            .add_startup_system_to_stage(StartupStage::PreStartup, set_default_display_config)
            .add_startup_system_to_stage(StartupStage::Startup, init_display)
            .add_system_to_stage(CoreStage::PostUpdate, render_objects)
            .add_system_to_stage(CoreStage::Last, swap_buffers);
    }
}