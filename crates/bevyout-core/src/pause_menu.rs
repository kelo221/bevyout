//! Fallout-style ESC pause menu selection state (engine-independent).

/// Ordered pause-menu entries matching the FO3 pause stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseMenuOption {
    Continue,
    Save,
    Load,
    Settings,
    Help,
    Quit,
}

impl PauseMenuOption {
    pub const ALL: [Self; 6] = [
        Self::Continue,
        Self::Save,
        Self::Load,
        Self::Settings,
        Self::Help,
        Self::Quit,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Continue => "Continue",
            Self::Save => "Save",
            Self::Load => "Load",
            Self::Settings => "Settings",
            Self::Help => "Help",
            Self::Quit => "Quit",
        }
    }

    /// Only Continue and Quit are wired; the rest render as disabled placeholders.
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Continue | Self::Quit)
    }
}

/// Action produced when an enabled menu option is confirmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseMenuAction {
    Continue,
    Quit,
}

impl PauseMenuAction {
    pub const fn from_option(option: PauseMenuOption) -> Option<Self> {
        match option {
            PauseMenuOption::Continue => Some(Self::Continue),
            PauseMenuOption::Quit => Some(Self::Quit),
            PauseMenuOption::Save
            | PauseMenuOption::Load
            | PauseMenuOption::Settings
            | PauseMenuOption::Help => None,
        }
    }
}

/// Keyboard/mouse selection cursor over the pause stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PauseMenuState {
    selected: usize,
}

impl Default for PauseMenuState {
    fn default() -> Self {
        Self::new()
    }
}

impl PauseMenuState {
    pub const fn new() -> Self {
        Self { selected: 0 }
    }

    pub const fn selected(self) -> PauseMenuOption {
        PauseMenuOption::ALL[self.selected]
    }

    pub const fn selected_index(self) -> usize {
        self.selected
    }

    pub fn select(&mut self, option: PauseMenuOption) {
        if let Some(index) = PauseMenuOption::ALL
            .iter()
            .position(|&entry| entry == option)
        {
            self.selected = index;
        }
    }

    /// Move selection up, wrapping through every entry (including disabled).
    pub fn move_up(&mut self) {
        if self.selected == 0 {
            self.selected = PauseMenuOption::ALL.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// Move selection down, wrapping through every entry (including disabled).
    pub fn move_down(&mut self) {
        self.selected = (self.selected + 1) % PauseMenuOption::ALL.len();
    }

    /// Confirm the current selection when it is an enabled action.
    pub const fn activate(self) -> Option<PauseMenuAction> {
        PauseMenuAction::from_option(self.selected())
    }
}

#[cfg(test)]
#[path = "tests/pause_menu.rs"]
mod tests;
