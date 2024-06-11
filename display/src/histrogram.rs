use macroquad::prelude::*;
use std::collections::HashMap;
use utilities::shift_f32_to_range;


pub struct Histogram {
    n_bins: u32,
    total_samples: u32,
    sample_bins: HashMap<u32, u32>,
    scr_x_start: u32,
    scr_y_start: u32,
    scr_x_end: u32,
    scr_y_end: u32,
    in_range_start: u32,
    in_range_end: u32,
    out_range_start: u32,
    out_range_end: u32,
}

impl Histogram {
    pub fn new(n_bins: u32,
               scr_x_start: u32,
               scr_y_start: u32,
               scr_x_end: u32,
               scr_y_end: u32,
               in_range_start: u32,
               in_range_end: u32,
               out_range_start: u32,
               out_range_end: u32) -> Result<Histogram, String> {
        if n_bins <= 0 {
            return Err("n_bins must be a positive integer!".to_string())
        }

        let mut sample_bins = HashMap::new();
        for i in 0..n_bins {
            sample_bins.insert(i, 0);
        }

        Ok (Histogram {
            n_bins,
            total_samples: 0,
            sample_bins,
            scr_x_start,
            scr_y_start,
            scr_x_end,
            scr_y_end,
            in_range_start,
            in_range_end,
            out_range_start,
            out_range_end
        })
    }

    pub fn add_value(&mut self, value: f32) {
        let shifted_val = shift_f32_to_range(
            value,
            self.in_range_start,
            self.in_range_end,
            self.out_range_start,
            self.out_range_end
        );
        let hist_width = self.scr_x_end - self.scr_x_start;
        let bin_width = hist_width / self.n_bins as f32;
        let mut hist_bin_pos = (shifted_val.expect("Error shifting value to new range!") / bin_width).floor();

        if hist_bin_pos == self.n_bins as f32 {
            hist_bin_pos -= 1.0;
        }

        self.total_samples += 1;
        let old_bin_count = self.sample_bins.get(hist_bin_pos);
        self.sample_bins.insert(hist_bin_pos, old_bin_count + 1);
    }
}