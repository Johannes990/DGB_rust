mod settings;

use display;

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
    display::run().await;
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
