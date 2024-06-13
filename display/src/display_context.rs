pub struct DisplayContext {
    current_window: DisplayWindow,
    previous_window: Option<DisplayWindow>,
}

#[derive(Clone, Copy)]
pub enum DisplayWindow {
    StartingPage,
    FirstPage,
    SecondPage,
    QuitDialogPage,
}

impl DisplayContext {
    pub fn new() -> Result<DisplayContext, String> {
        Ok(DisplayContext { current_window: DisplayWindow::StartingPage, previous_window: None })
    }

    pub fn set_current_window(&mut self, new_current_window: DisplayWindow) {
        self.current_window = new_current_window;
    }

    pub fn set_previous_window(&mut self, new_previous_window: Option<DisplayWindow>) {
        self.previous_window = new_previous_window;
    }

    pub fn get_current_window(&self) -> DisplayWindow {
        self.current_window
    }

    pub fn get_previous_window(&self) -> DisplayWindow {
        self.previous_window.unwrap()
    }
}
