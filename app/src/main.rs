use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::window::next_frame;
use display;
use probability::source::Xorshift128Plus;
use distributions::beta_distribution::BetaDistribution;
use distributions::ProbabilityDistribution;
use utilities::generate_seed;


fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Dynamic Graph Builder".to_owned(),
        window_height: 500,
        window_width: 500,
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
    let mut x_values:Vec<f32> = vec![250.0];
    let seed = generate_seed();
    let mut source = Xorshift128Plus::new(seed);
    let beta_dist = BetaDistribution::new(20.0, 20.0, 0.0, 500.0).expect("Failed to create beta distribution!");

    loop {
        display::initialize().await;

        if is_key_pressed(KeyCode::A) {
            let random = beta_dist.generate_random_sample(&mut source);
            println!("Random number {} generated", random);
            x_values.push(random as f32);
        }
        display::draw_circles_from_vec_values(&x_values).await;

        next_frame().await
    }
}
