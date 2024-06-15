mod histogram;
mod start_page;
mod display_context;
mod first_page;
mod options_page;
mod elements;
mod quit_page;

use macroquad::prelude::*;
use display_context::{DisplayContext, DisplayWindow};
use distributions::distribution_class::DistributionClass;
use distributions::distribution_type::DistributionType;
use settings::distribution_settings::DistributionSettings;
use crate::elements::palette::{FONT_OPEN_SANS_BUTTON, FONT_OPEN_SANS_OPTIONS};
use crate::elements::slider::{Slider, SliderType};
use crate::elements::selection_list::SelectionList;

pub async fn run() {
    let mut current_display = DisplayContext::new().unwrap();
    let mut distribution_settings = DistributionSettings::new();
    let user_exit_requested = false;

    let font = load_ttf_font("C:\\Users\\johan\\RustroverProjects\\DGB_rust\\assets\\fonts\\open_sans\\OpenSans_Medium.ttf").await.unwrap();
    let button_text_params = TextParams { font: Some(&font), font_size: 20, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: FONT_OPEN_SANS_BUTTON };
    let option_text_params = TextParams { font: Some(&font), font_size: 18, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: FONT_OPEN_SANS_OPTIONS };

    let mut slider_x1 = Slider::new(25.0, 350.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 1", &option_text_params);
    let mut slider_y1 = Slider::new(185.0, 350.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 1", &option_text_params);
    let mut slider_x2 = Slider::new(25.0, 410.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 2", &option_text_params);
    let mut slider_y2 = Slider::new(185.0, 410.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 2", &option_text_params);
    let mut slider_x3 = Slider::new(25.0, 470.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 3", &option_text_params);
    let mut slider_y3 = Slider::new(185.0, 470.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 3", &option_text_params);
    let mut slider_x4 = Slider::new(25.0, 530.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 4", &option_text_params);
    let mut slider_y4 = Slider::new(185.0, 530.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 4", &option_text_params);
    let mut distro:Vec<DistributionClass> = Vec::new();
    distro.push(DistributionClass { payload: DistributionType::Cauchy, name: "Cauchy".to_string(), });
    distro.push(DistributionClass { payload: DistributionType::Exponent, name: "Exponent".to_string(), });
    distro.push(DistributionClass { payload: DistributionType::Gaussian, name: "Gaussian".to_string(), });
    let mut checklist = SelectionList::new(450.0, 300.0, 20.0, distro, "Selection list".to_string(), &option_text_params);


    loop {
        let current_window = current_display.get_current_window();

        match current_window {
            DisplayWindow::StartingPage => {
                if let Some(start_page_element) = start_page::show_page(&button_text_params).await {
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
                if let Some(first_page_element) = first_page::show_page(&button_text_params, &distribution_settings.clone()).await {
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
                if let Some(second_page_element) = options_page::show_page(&button_text_params,
                                                                           &option_text_params,
                                                                           &mut distribution_settings,
                                                                           &mut slider_x1,
                                                                           &mut slider_y1,
                                                                           &mut slider_x2,
                                                                           &mut slider_y2,
                                                                           &mut slider_x3,
                                                                           &mut slider_y3,
                                                                           &mut slider_x4,
                                                                           &mut slider_y4,
                                                                           &mut checklist).await {
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
                if let Some(quit_page_element) = quit_page::show_page(&button_text_params, &user_exit_requested).await {
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
