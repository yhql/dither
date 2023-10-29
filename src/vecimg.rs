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

        // let pixels: Vec<u8> = img
        //     .pixels()
        //     // .map(|x| x.2.to_rgb().0[0] as f32 / 256.)
        //     .map(|x| x.2.to_luma().0[0] as f32 / 256.)
        //     // .map(|x| x.sqrt())
        //     .map(srgb_to_linear)
        //     .map(|x| (x * 256.) as u8)
        //     .collect();
        // let bins = bin_lum(pixels);
        // let threshold = bins
        //     .iter()
        //     .enumerate()
        //     .fold(0., |acc, (i, bin)| acc + (i as f32 / *bin as f32))
        //     / bins.iter().sum::<u32>() as f32;

        let pixels: Vec<f32> = img
            .pixels()
            // .map(|x| x.2.to_rgb().0[0] as f32 / 256.)
            .map(|x| x.2.to_luma().0[0] as f32 / 256.)
            .map(srgb_to_linear)
            .collect();

        // TODO: Use Otsu's method to find a threshold?
        // the doom pictures always look too dark...
        let threshold = 0.5;
        // let threshold = pixels.iter().sum::<f32>() / pixels.len() as f32;
        // let threshold = pixels.iter().fold(0., |acc, pixel| if pixel > &acc { *pixel } else { acc }) / 2.;

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
