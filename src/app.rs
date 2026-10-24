use std::time::Instant;

use glam::Vec3;
use minifb::Key;
use rfd::FileDialog;

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::molecule::{Molecule, make_examples, molecule_name};
use crate::render::{Color, Framebuffer};
use crate::window::AppWindow;

use crate::scene;
use crate::ui;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SS_BQ: usize = 1;
const SS_HQ: usize = 3;

const MENU_OPEN: usize = 1;
const MENU_EXAMPLE: usize = 2;

pub fn run() {
    App::new().run();
}

fn reframe(m: &Molecule) -> (Vec3, Camera) {
    // reframe the cam around the new mol
    let points: Vec<Vec3> = m.atoms.iter().map(|a| a.position).collect();
    let (centroid, max_r) = crate::math::bounding_sphere(&points);

    let mut cam = Camera::new(Vec3::ZERO, max_r * 4.0, 60.0, 0.1, 100.0);
    cam.set_distance_bounds(max_r * 1.2, max_r * 8.0);
    (-centroid, cam)
}

struct App {
    window: AppWindow,
    framebuffer: Framebuffer,
    render_fb: Framebuffer,
    ss: usize,

    sphere: Mesh,
    cylinder: Mesh,
    camera: Camera,

    examples: Vec<(&'static str, Molecule)>,
    mol: Option<Molecule>,
    shift: Vec3,

    show_examples: bool,
    needs_redraw: bool,
    last_hover: Option<usize>,

    frames: u32,
    last_print: Instant,
}

impl App {
    fn new() -> Self {
        let ss = SS_BQ;
        Self {
            window: AppWindow::new("Quantik", WIDTH, HEIGHT),
            framebuffer: Framebuffer::new(WIDTH, HEIGHT),
            render_fb: Framebuffer::new(WIDTH * ss, HEIGHT * ss),
            ss,

            sphere: Mesh::generate_uv_sphere(1.0, 16, 16),
            cylinder: Mesh::generate_cylinder(1.0, 1.0, 16),
            camera: Camera::new(Vec3::ZERO, 10.0, 60.0, 0.1, 100.0),

            examples: make_examples(),
            mol: None,
            shift: Vec3::ZERO,

            show_examples: false,
            needs_redraw: true,
            last_hover: None,

            frames: 0,
            last_print: Instant::now(),
        }
    }

    fn run(&mut self) {
        while self.window.active() {
            if self.window.is_key_pressed(Key::Escape) {
                if self.show_examples {
                    self.show_examples = false;
                    self.needs_redraw = true;
                } else {
                    break;
                }
            }

            self.handle_menu();
            self.handle_example_keys();

            let size = self.window.get_size();
            self.sync_size(size);

            let moved = self.update_camera();
            self.update_quality(size, moved);

            let mouse_pos = self.window.get_mouse_pos();
            let hovered = self.hovered_atom(mouse_pos);

            if self.needs_redraw {
                self.redraw(mouse_pos);
            }

            if hovered != self.last_hover {
                self.needs_redraw = true;
                self.last_hover = hovered;
            }

            self.present();
            // TODO: Remove in release build
            self.print_fps();
        }
    }

    fn load_molecule(&mut self, mol: Molecule) {
        let (shift, camera) = reframe(&mol);
        self.shift = shift;
        self.camera = camera;

        let new_title = format!(
            "Quantik - {} ({})",
            molecule_name(mol.cid),
            mol.cid
        );
        self.window.set_title(&new_title);

        self.mol = Some(mol);
        self.last_hover = None;
        self.show_examples = false;
        self.needs_redraw = true;
    }

    fn handle_menu(&mut self) {
        match self.window.is_menu_pressed() {
            Some(MENU_OPEN) => {
                if let Some(file) = FileDialog::new()
                    .add_filter("SDF File", &["sdf", "txt"])
                    .pick_file()
                    && let Ok(new_mol) = Molecule::from_sdf_file(file.to_str().unwrap_or(""))
                {
                    self.load_molecule(new_mol);
                }
            },
            Some(MENU_EXAMPLE) => {
                self.show_examples = !self.show_examples;
                self.needs_redraw = true;
            },
            _ => {}
        }
    }

    fn handle_example_keys(&mut self) {
        const KEYS: [Key; 9] = [
            Key::Key1, Key::Key2, Key::Key3,
            Key::Key4, Key::Key5, Key::Key6,
            Key::Key7, Key::Key8, Key::Key9,
        ];
        
        if !self.show_examples {
            return;
        }

        for (i, key) in KEYS.iter().enumerate() {
            if self.window.is_key_pressed(*key)
                && let Some(new_mol) = self.examples.get(i).map(|(_, template)| template.clone())
            {
                self.load_molecule(new_mol);
            }
        }
    }

    fn sync_size(&mut self, (w, h): (usize, usize)) {
        if self.framebuffer.width != w || self.framebuffer.height != h {
            self.framebuffer.resize(w, h); // resize content of fb (on macOS it's when releasing the resize handle)
            self.render_fb.resize(w * self.ss, h * self.ss);
            self.needs_redraw = true;
        }
    }

    fn update_camera(&mut self) -> bool {
        let moved = self.mol.is_some() && self.camera.handle_input(&self.window);
        if moved {
            self.needs_redraw = true;
        }
        moved
    }

    fn update_quality(&mut self, (w, h): (usize, usize), moved: bool) {
        let want = if moved { SS_BQ } else { SS_HQ };
        if want != self.ss {
            self.ss = want;
            self.render_fb.resize(w * want, h * want);
            self.needs_redraw = true;
        }
    } 

    fn hovered_atom(&self, mouse_pos: Option<(f32, f32)>) -> Option<usize> {
        let (mx, my) = mouse_pos?;
        let x = (mx * self.ss as f32) as usize;
        let y = (my * self.ss as f32) as usize;

        if x < self.render_fb.width && y < self.render_fb.height {
            let atom_idx = self.render_fb.atom_ids[y * self.render_fb.width + x];
            (atom_idx != usize::MAX).then_some(atom_idx)
        } else {
            None
        }
    }

    fn redraw(&mut self, mouse_pos: Option<(f32, f32)>) {
        self.framebuffer.clear(Color::BLACK);
        self.render_fb.clear(Color::BLACK);

        match &self.mol {
            Some(m) => {
                scene::draw_molecule(
                    &mut self.render_fb,
                    m,
                    self.shift,
                    &self.camera,
                    &self.sphere,
                    &self.cylinder,
                );
                self.render_fb.resolve_into(&mut self.framebuffer, self.ss);

                if let (Some(idx), Some((mx, my))) = (self.last_hover, mouse_pos) {
                    ui::draw_atom_tooltip(&mut self.framebuffer, &m.atoms[idx], idx, mx, my);
                }
                ui::draw_molecule_info(&mut self.framebuffer, m);
            }
            None => ui::draw_welcome(&mut self.framebuffer),
        }

        if self.show_examples {
            ui::draw_examples_menu(&mut self.framebuffer, &self.examples);
        }

        self.needs_redraw = false;
        self.frames += 1;
    }

    fn present(&mut self) {
        let buffer = self.framebuffer.to_u32_buffer();
        self.window
            .update_with_buffer(&buffer, self.framebuffer.width, self.framebuffer.height);
    }

    fn print_fps(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_print).as_secs_f32() >= 1.0 {
            println!("FPS: {}", self.frames);
            self.frames = 0;
            self.last_print = now;
        }
    }
}
