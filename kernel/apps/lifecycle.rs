
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AppState {
    Empty = 0,
    Registered = 1,
    Loaded = 2,
    Running = 3,
    Paused = 4,
    Closed = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition,
}

impl AppState {
    pub fn can_load(self) -> bool {
        matches!(self, Self::Registered | Self::Closed)
    }

    pub fn can_start(self) -> bool {
        matches!(self, Self::Loaded | Self::Paused)
    }

    pub fn can_pause(self) -> bool {
        self == Self::Running
    }

    pub fn can_close(self) -> bool {
        matches!(
            self,
            Self::Registered
                | Self::Loaded
                | Self::Running
                | Self::Paused
        )
    }
}

pub fn load(state: &mut AppState) -> Result<(), LifecycleError> {
    if !state.can_load() {
        return Err(LifecycleError::InvalidTransition);
    }

    *state = AppState::Loaded;
    Ok(())
}

pub fn start(state: &mut AppState) -> Result<(), LifecycleError> {
    if !state.can_start() {
        return Err(LifecycleError::InvalidTransition);
    }

    *state = AppState::Running;
    Ok(())
}

pub fn pause(state: &mut AppState) -> Result<(), LifecycleError> {
    if !state.can_pause() {
        return Err(LifecycleError::InvalidTransition);
    }

    *state = AppState::Paused;
    Ok(())
}

pub fn close(state: &mut AppState) -> Result<(), LifecycleError> {
    if !state.can_close() {
        return Err(LifecycleError::InvalidTransition);
    }

    *state = AppState::Closed;
    Ok(())
}
