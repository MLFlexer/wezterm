use wezterm_term::TerminalSize;

use crate::{tab::{PaneNode, Tab, Tree}, window::Window};

pub struct MuxSessionState {
    pub(crate) windows: Vec<WindowSessionState>
}

pub struct WindowSessionState {
    pub(crate) tabs: Vec<TabSessionState>,
    pub(crate) size: TerminalSize,
    pub(crate) workspace: String,
    pub(crate) title: String,
}

pub struct TabSessionState {
    pub(crate) pane: PaneNode,
    //size: TerminalSize,
    pub(crate) title: String,
}
