use macroquad::prelude::*;
use std::collections::HashMap;
use utilities::shift_f32_to_range;


pub struct Histogram {
    bin_count: u32,
    total_samples: u32,
    sample_bins: HashMap<u32, u32>,
    hist_x_start: f32,
    hist_y_start: f32,
    hist_x_end: f32,
    hist_y_end: f32,
    in_range_start: u32,
    in_range_end: u32,
    out_range_start: u32,
    out_range_end: u32,
    width: f32,
    height: f32,
    bin_width: f32,
    bin_padding: f32,
    background_color: Color,
    bin_color: Color
}

impl Histogram {
    pub fn new(bin_count: u32,
               hist_x_start: f32,
               hist_y_start: f32,
               hist_x_end: f32,
               hist_y_end: f32,
               in_range_start: u32,
               in_range_end: u32,
               bin_padding: f32,
               background_color: Color,
               bin_color: Color) -> Result<Histogram, String> {
        if bin_count <= 0 {
            return Err("n_bins must be a positive integer!".to_string())
        }
        let width = hist_x_end - hist_x_start;
        let height = hist_y_end - hist_y_start;
        if (bin_count as f32 * ((2.0 * bin_padding) + 1.0)) > width || (bin_padding + 1.0) > height {
            return Err("Histogram dimensions too small to show with given padding size!".to_string())
        }

        let mut sample_bins = HashMap::new();
        for i in 0..bin_count {
            sample_bins.insert(i, 0);
        }

        Ok (Histogram {
            bin_count,
            total_samples: 0,
            sample_bins,
            hist_x_start,
            hist_y_start,
            hist_x_end,
            hist_y_end,
            in_range_start,
            in_range_end,
            out_range_start: hist_x_start as u32,
            out_range_end: hist_x_end as u32,
            width,
            height,
            bin_width: width / bin_count as f32,
            bin_padding,
            background_color,
            bin_color
        })
    }

    pub fn add_value(&mut self, value: f32) {
        let shifted_val = shift_f32_to_range(
            value,
            self.in_range_start,
            self.in_range_end,
            self.out_range_start,
            self.out_range_end
        ).unwrap_or(0.0);
        let mut hist_bin_pos = ((shifted_val - self.hist_x_start) / self.bin_width).floor() as u32;

        if hist_bin_pos == self.bin_count {
            hist_bin_pos -= 1;
        }

        println!("adding value: {}\nshifted value: {}\nhistogram bin nr: {}",value, shifted_val, hist_bin_pos);

        self.total_samples += 1;
        let old_bin_count = self.sample_bins.get(&hist_bin_pos).unwrap();
        self.sample_bins.insert(hist_bin_pos, old_bin_count + 1);
    }

    pub fn print_bins(&self) {
        for i in self.sample_bins.keys() {
            println!("sample bin {} has {} values", i, self.sample_bins.get(i).unwrap())
        }
    }

    pub async fn show(&self) {
        draw_rectangle(
            self.hist_x_start,
            self.hist_y_start,
            self.width,
            self.height,
            self.background_color
        );

        if self.total_samples == 0 {
            return
        }

        let bin_max_height = self.height - self.bin_padding;
        for bin_idx in 0..self.bin_count {
            let bin_rel_height = *self.sample_bins.get(&bin_idx).unwrap() as f32 / self.total_samples as f32;
            let bin_start_x = (self.bin_width * bin_idx as f32) + self.bin_padding;
            let bin_end_x = bin_start_x + self.bin_width - (2.0 * self.bin_padding);
            let bin_start_y = self.hist_y_end - (bin_max_height * bin_rel_height);
            draw_rectangle(bin_start_x + self.hist_x_start, bin_start_y, bin_end_x - bin_start_x, self.hist_y_end - bin_start_y, self.bin_color);
        }
    }
}