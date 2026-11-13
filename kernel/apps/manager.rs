
use super::lifecycle::{
    self,
    AppState,
};

pub const MAX_APPS: usize = 16;
pub const FIRST_APP_ID: u32 = 1;

#[derive(Clone, Copy)]
pub struct AppDescriptor {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub package_id: &'static str,
}

#[derive(Clone, Copy)]
pub struct AppRecord {
    pub id: u32,
    pub descriptor: AppDescriptor,
    pub state: AppState,
    pub window_id: Option<u32>,
}

impl AppRecord {
    const EMPTY: Self = Self {
        id: 0,
        descriptor: AppDescriptor {
            name: "",
            version: "",
            description: "",
            package_id: "",
        },
        state: AppState::Empty,
        window_id: None,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManagerError {
    CapacityReached,
    AppNotFound,
    InvalidTransition,
    InvalidId,
    DuplicatePackage,
}

pub struct AppManager {
    apps: [AppRecord; MAX_APPS],
    next_id: u32,
}

impl AppManager {
    pub const fn new() -> Self {
        Self {
            apps: [AppRecord::EMPTY; MAX_APPS],
            next_id: FIRST_APP_ID,
        }
    }

    fn find_index(&self, id: u32) -> Option<usize> {
        self.apps.iter().position(|app| {
            app.id == id && app.state != AppState::Empty
        })
    }

    pub fn register(
        &mut self,
        descriptor: AppDescriptor,
    ) -> Result<u32, ManagerError> {
        if self.find_package(descriptor.package_id).is_some() {
            return Err(ManagerError::DuplicatePackage);
        }

        let index = self.apps
            .iter()
            .position(|app| app.state == AppState::Empty)
            .ok_or(ManagerError::CapacityReached)?;

        let id = self.next_id;

        let next_id = id
            .checked_add(1)
            .ok_or(ManagerError::InvalidId)?;

        self.apps[index] = AppRecord {
            id,
            descriptor,
            state: AppState::Registered,
            window_id: None,
        };

        self.next_id = next_id;

        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&AppRecord> {
        self.find_index(id).map(|index| &self.apps[index])
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut AppRecord> {
        let index = self.find_index(id)?;
        Some(&mut self.apps[index])
    }

    pub fn find_package(
        &self,
        package_id: &str,
    ) -> Option<&AppRecord> {
        self.apps.iter().find(|app| {
            app.state != AppState::Empty
                && app.descriptor.package_id == package_id
        })
    }

    pub fn load(&mut self, id: u32) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        lifecycle::load(&mut app.state)
            .map_err(|_| ManagerError::InvalidTransition)
    }

    pub fn start(&mut self, id: u32) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        lifecycle::start(&mut app.state)
            .map_err(|_| ManagerError::InvalidTransition)
    }

    pub fn pause(&mut self, id: u32) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        lifecycle::pause(&mut app.state)
            .map_err(|_| ManagerError::InvalidTransition)
    }

    pub fn close(&mut self, id: u32) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        lifecycle::close(&mut app.state)
            .map_err(|_| ManagerError::InvalidTransition)?;

        app.window_id = None;

        Ok(())
    }

    // Automatically loads and starts an application.
    // Already-running applications are not started twice.
    pub fn activate(&mut self, id: u32) -> Result<(), ManagerError> {
        let state = self.get(id)
            .ok_or(ManagerError::AppNotFound)?
            .state;

        match state {
            AppState::Registered | AppState::Closed => {
                self.load(id)?;
                self.start(id)?;
            }

            AppState::Loaded | AppState::Paused => {
                self.start(id)?;
            }

            AppState::Running => {}

            AppState::Empty => {
                return Err(ManagerError::AppNotFound);
            }
        }

        Ok(())
    }

    pub fn set_window(
        &mut self,
        id: u32,
        window_id: u32,
    ) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        app.window_id = Some(window_id);
        Ok(())
    }

    pub fn clear_window(
        &mut self,
        id: u32,
    ) -> Result<(), ManagerError> {
        let app = self.get_mut(id)
            .ok_or(ManagerError::AppNotFound)?;

        app.window_id = None;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.apps.iter()
            .filter(|app| app.state != AppState::Empty)
            .count()
    }

    pub fn slot(&self, index: usize) -> Option<&AppRecord> {
        self.apps.get(index).filter(|app| {
            app.state != AppState::Empty
        })
    }
}
