mod histogram;
mod start_page;
mod display_context;
mod first_page;
mod options_page;
mod elements;
mod quit_page;
use macroquad::prelude::*;
use display_context::{DisplayContext, DisplayWindow};


pub async fn run() {
    let mut current_display = DisplayContext::new().unwrap();
    let arial_font = load_ttf_font("assets/fonts/arial/arial.ttf").await.unwrap();
    let mut user_exit_requested = false;

    loop {
        let current_window = current_display.get_current_window();

        match current_window {
            DisplayWindow::StartingPage => {
                if let Some(start_page_element) = start_page::show_page().await {
                    match start_page_element {
                        1 => {
                            println!("Mouse selected BUTTON 1...");
                            current_display.set_current_window(DisplayWindow::FirstPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::StartingPage));
                        },
                        2 => {
                            println!("Options page selected...");
                            current_display.set_current_window(DisplayWindow::OptionsPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::StartingPage));
                        },
                        255 => {
                            println!("QUIT dialog page selected...");
                            current_display.set_current_window(DisplayWindow::QuitDialogPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::StartingPage));
                        },
                        _ => {},
                    }
                }
            },
            DisplayWindow::FirstPage => {
                if let Some(first_page_element) = first_page::show_page(&arial_font).await {
                    match first_page_element {
                        1 => {
                            println!("BACK BUTTON pressed from page one...");
                            current_display.set_current_window(DisplayWindow::StartingPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::FirstPage));
                        },
                        255 => {
                            println!("QUIT BUTTON pressed from page one...");
                            current_display.set_current_window(DisplayWindow::QuitDialogPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::FirstPage));
                        },
                        _ => {},
                    }
                }
            },
            DisplayWindow::OptionsPage => {
                if let Some(second_page_element) = options_page::show_page().await {
                    match second_page_element {
                        1 => {
                            println!("BACK BUTTON pressed from options page...");
                            current_display.set_current_window(DisplayWindow::StartingPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::OptionsPage));
                        },
                        255 => {
                            println!("QUIT BUTTON pressed from options page...");
                            current_display.set_current_window(DisplayWindow::QuitDialogPage);
                            current_display.set_previous_window(Option::from(DisplayWindow::OptionsPage));
                        }
                        _ => {},
                    }
                }
            },
            DisplayWindow::QuitDialogPage => {
                if let Some(quit_page_element) = quit_page::show_page(&user_exit_requested).await {
                    match quit_page_element {
                        0 => {
                            println!("QUITTING...");
                            break;
                        },
                        254 => {
                            println!("RETURNING TO PREVIOUS PAGE...");
                            current_display.set_current_window(current_display.get_previous_window());
                            current_display.set_previous_window(None);
                        },
                        _ => {},
                    }
                }
            }
        }

        next_frame().await;
    }
}

pub async fn draw_circles_from_vec_values(positions: &Vec<f32>) {
    for x_val in positions {
        draw_circle(*x_val, 250.0, 3.0, BLACK);
        draw_circle(*x_val, 250.0, 1.0, RED);
    }
}
