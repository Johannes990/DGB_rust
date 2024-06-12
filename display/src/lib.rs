mod histogram;
mod start_page;
mod display_context;
mod first_page;
mod second_page;
mod elements;
use macroquad::prelude::*;
use display_context::{DisplayContext, DisplayWindow};


pub async fn run() {
    let mut current_display = DisplayContext::new().unwrap();
    let arial_font = load_ttf_font("assets/fonts/arial/arial.ttf").await.unwrap();

    loop {
        let current_window = current_display.get_window();

        match current_window {
            DisplayWindow::StartingPage => {
                if let Some(start_page_element) = start_page::show_page().await {
                    match start_page_element {
                        1 => {
                            println!("Mouse selected BUTTON 1...");
                            current_display.switch_window(DisplayWindow::FirstPage);
                        },
                        2 => {
                            println!("Mouse selected BUTTON 2...");
                            current_display.switch_window(DisplayWindow::SecondPage);
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
                            current_display.switch_window(DisplayWindow::StartingPage);
                        },
                        _ => {},
                    }
                }
            },
            DisplayWindow::SecondPage => {
                if let Some(second_page_element) = second_page::show_page().await {
                    match second_page_element {
                        1 => {
                            println!("BACK BUTTON pressed from page two...");
                            current_display.switch_window(DisplayWindow::StartingPage);
                        },
                        _ => {},
                    }
                }
            },
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
