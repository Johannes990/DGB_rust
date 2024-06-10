use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::window::next_frame;
use display;
use probability::source::Xorshift128Plus;
use distributions::uniform_distribution::UniformDistribution;
use distributions::log_normal_distribution::LogNormalDistribution;
use distributions::gamma_distribution::GammaDistribution;
use distributions::ProbabilityDistribution;


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

fn generate_seed() -> [u64; 2] {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time error");

    let nanos = since_epoch.as_nanos();
    [(nanos & 0xFFFFFFFFFFFFFFFF) as u64, (nanos >> 64) as u64]
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut x_values:Vec<f32> = vec![250.0];
    let seed = generate_seed();
    let mut source = Xorshift128Plus::new(seed);
    let uniform_dist = UniformDistribution::new(0.0, 500.0).expect("Failed to create uniform distribution!");
    let log_norm_dist = LogNormalDistribution::new(3.3, 0.75).expect("Failed to create lognormal distribution");
    let gamma_dist = GammaDistribution::new(1.0, 145.0).expect("Failed to create gamma distribution");

    loop {
        display::initialize().await;

        if is_key_pressed(KeyCode::A) {
            let random = gamma_dist.generate_random_sample(&mut source);
            println!("Random number {} generated", random);
            x_values.push(random as f32);
        }
        display::draw_circles_from_vec_values(&x_values).await;

        next_frame().await
    }
}
