mod distribution_loader;
use macroquad::prelude::{load_ttf_font, TextParams};
use display;
use display::app_state::AppState;
use display::values_and_constants::palette::{FONT_OPEN_SANS_BUTTON, FONT_OPEN_SANS_OPTIONS};
use crate::distribution_loader::DistributionLoader;

fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Dynamic Graph Builder".to_owned(),
        window_height: 600,
        window_width: 700,
        high_dpi: false,
        window_resizable: true,
        fullscreen: false,
        icon: None,
        sample_count: 0,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("C:\\Users\\johan\\RustroverProjects\\DGB_rust\\assets\\fonts\\open_sans\\OpenSans_Medium.ttf").await.unwrap();
    let button_text_params = TextParams { font: Some(&font), font_size: 20, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: FONT_OPEN_SANS_BUTTON };
    let option_text_params = TextParams { font: Some(&font), font_size: 18, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: FONT_OPEN_SANS_OPTIONS };
    let mut app_state = AppState::new(option_text_params.clone()).await;
    let mut distribution_loader = DistributionLoader::new(&app_state);

    let x_ax_dist = distribution_loader.get_x_axis_distribution(&app_state);
    let y_ax_dist = distribution_loader.get_y_axis_distribution(&app_state);

    display::run(&mut app_state, &button_text_params).await;
    /*let mut x_values:Vec<f32> = vec![250.0];
    let seed = generate_seed();
    let mut source = Xorshift128Plus::new(seed);
    let beta_dist = GaussianDistribution::new(250.0, 140.0).expect("Failed to create beta distribution!");
    let mut histogram = Histogram::new(
        5,
        0.0,
        0.0,
        100.0,
        100.0,
        0,
        500,
        2.0,
        MAROON,
        RED
    ).unwrap();

    loop {
        display::initialize().await;

        if is_key_pressed(KeyCode::A) {
            let random = beta_dist.generate_random_sample(&mut source);
            println!("Random number {} generated", random);
            x_values.push(random as f32);
            histogram.add_value(random as f32);
        }
        display::draw_circles_from_vec_values(&x_values).await;
        histogram.print_bins();
        histogram.show().await;

        next_frame().await
    }*/
}
