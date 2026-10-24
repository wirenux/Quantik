use minifb::{Key, Menu, MouseButton, MouseMode, Window, WindowOptions, MENU_KEY_CTRL};

pub struct AppWindow {
    inner: Window,
    menu_handle: Option<minifb::MenuHandle>,
}

impl AppWindow {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(120);

        let window_menu = Menu::new("Window").unwrap();
        let _window_handle = window.add_menu(&window_menu);

        let mut menu = Menu::new("File").unwrap();
        menu.add_item("Open...", 1) // ID 1 for Open
            .shortcut(Key::O, MENU_KEY_CTRL) // ctrl even for macOS 'cause minifb make the change
            .build();

        let mut mol_example_menu = Menu::new("Examples").unwrap();
        mol_example_menu
            .add_item("Show list", 2)
            .shortcut(Key::E, MENU_KEY_CTRL)
            .build();

        let menu_handle = Some(window.add_menu(&menu));
        let _mol_example_menu_handle = window.add_menu(&mol_example_menu);

        Self {
            inner: window,
            menu_handle,
        }
    }

    pub fn active(&self) -> bool {
        self.inner.is_open()
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.inner.get_size()
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32], width: usize, height: usize) {
        self.inner
            .update_with_buffer(buffer, width, height)
            .unwrap();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.inner.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.inner.is_key_pressed(key, minifb::KeyRepeat::No)
    }

    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.inner.get_mouse_pos(MouseMode::Discard)
    }

    pub fn is_mouse_down(&self, left: bool) -> bool {
        self.inner.get_mouse_down(if left {
            MouseButton::Left
        } else {
            MouseButton::Right
        })
    }

    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        self.inner.get_scroll_wheel()
    }

    pub fn set_title(&mut self, title: &str) {
        self.inner.set_title(title);
    }

    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        self.inner.is_menu_pressed()
    }
}
