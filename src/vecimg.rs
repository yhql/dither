use crate::algs::Filter;
use core::ops::{Index, IndexMut};
use image::{DynamicImage, GenericImageView, Pixel};

pub struct VecImg {
    w: usize,
    _h: usize,
    pub pixels: Vec<f32>,
    pub threshold: f32,
}

fn srgb_to_linear(l: f32) -> f32 {
    if l <= 0.04045 {
        l / 12.92
    } else {
        ((l + 0.055) / 1.055).powf(2.4)
    }
}

// Compute occurences of each pixel intensity
fn bin_lum(pixels: Vec<u8>) -> [u32; 256] {
    let mut res = [1u32; 256];
    for p in pixels {
        res[p as usize] += 1;
    }
    res
}

impl From<DynamicImage> for VecImg {
    fn from(img: DynamicImage) -> Self {
        let (w, h) = img.dimensions();

        // Perform srgb_to_linear from the RGB image and convert to luminance
        // Although `image` can do `.to_luma`, applying srgb_to_linear before
        // leads to useless conversions, so everything done at once here.
        //
        // https://www.w3.org/WAI/GL/wiki/Relative_luminance

        let pixels: Vec<f32> = img
            .pixels()
            .map(|(_, _, rgb)| {
                0.2126 * srgb_to_linear(rgb.0[0] as f32 / 255.)
                    + 0.7152 * srgb_to_linear(rgb.0[1] as f32 / 255.)
                    + 0.0722 * srgb_to_linear(rgb.0[2] as f32 / 255.)
            })
            .collect();

        // TODO: Use Otsu's method to find a threshold?
        // the doom pictures always look too dark...
        // let threshold = 0.5;
        // let threshold = pixels.iter().sum::<f32>() / pixels.len() as f32;
        // let threshold = pixels.iter().fold(0., |acc, pixel| if pixel > &acc { *pixel } else { acc }) / 2.;

        let bins = bin_lum(pixels.iter().map(|p| (p * 255.) as u8).collect());
        let threshold = bins
            .iter()
            .enumerate()
            .fold(0., |acc, (i, bin)| acc + (i as f32 / *bin as f32))
            / bins.iter().sum::<u32>() as f32;

        VecImg {
            w: w as usize,
            _h: h as usize,
            pixels,
            threshold,
        }
    }
}

impl Index<(u32, u32)> for VecImg {
    type Output = f32;
    fn index(&self, idx: (u32, u32)) -> &Self::Output {
        &self.pixels[(idx.0 as usize) + (idx.1 as usize) * self.w]
    }
}

impl Index<(usize, usize)> for VecImg {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.index((idx.0 as u32, idx.1 as u32))
    }
}

impl IndexMut<(u32, u32)> for VecImg {
    fn index_mut(&mut self, idx: (u32, u32)) -> &mut Self::Output {
        &mut self.pixels[(idx.0 as usize) + (idx.1 as usize) * self.w]
    }
}

impl IndexMut<(usize, usize)> for VecImg {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        self.index_mut((idx.0 as u32, idx.1 as u32))
    }
}

impl VecImg {
    // (quantized, error)
    fn quantize(&self, p: f32) -> (f32, f32) {
        if p <= self.threshold {
            (0., p)
        } else {
            (1., p - 1.)
        }
    }

    pub fn apply_filter<const N: usize>(&mut self, at_x: usize, at_y: usize, filter: &Filter<N>) {
        let n = filter.matrix.len();
        let (q, e) = self.quantize(self[(at_x, at_y)]);

        for i in 0..n {
            for j in 0..n {
                self[(at_x - n / 2 + i, at_y - n / 2 + j)] +=
                    (filter.matrix[i][j] * e) / filter.div;
            }
        }
        self[(at_x, at_y)] = q;
    }
}
