
pub mod lockscreen {
    pub struct LockScreen {
        pub locked: bool,
        pub username: String,
    }

    impl LockScreen {
        pub fn new(username: String) -> Self {
            Self {
                locked: true,
                username,
            }
        }

        pub fn unlock(&mut self, authenticated: bool) -> bool {
            if authenticated {
                self.locked = false;
            }

            !self.locked
        }
    }
}

pub mod desktop {
    pub struct Desktop {
        pub visible: bool,
        pub start_menu_open: bool,
        pub taskbar_visible: bool,
    }

    impl Desktop {
        pub fn new() -> Self {
            Self {
                visible: false,
                start_menu_open: false,
                taskbar_visible: true,
            }
        }

        pub fn show(&mut self) {
            self.visible = true;
        }

        pub fn toggle_start_menu(&mut self) {
            self.start_menu_open = !self.start_menu_open;
        }
    }
}

fn main() {
    println!("BloxOS Desktop Environment");
    println!("Version: 0.1.0");
}
