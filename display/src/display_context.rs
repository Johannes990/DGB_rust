pub struct DisplayContext {
    current_window: DisplayWindow,
}

#[derive(Clone, Copy)]
pub enum DisplayWindow {
    StartingPage,
    FirstPage,
    SecondPage,
}

impl DisplayContext {
    pub fn new() -> Result<DisplayContext, String> {
        Ok(DisplayContext { current_window: DisplayWindow::StartingPage })
    }

    pub fn switch_window(&mut self, new_window: DisplayWindow) {
        self.current_window = new_window;
    }

    pub fn get_window(&self) -> DisplayWindow {
        self.current_window
    }
}
