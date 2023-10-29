use image::GenericImageView;
use std::path::PathBuf;

mod algs;
mod vecimg;

use algs::*;
use vecimg::VecImg;

fn dither_file<const N: usize>(filename: &PathBuf, f: &Filter<N>) {
    let img = image::open(filename).unwrap();
    let (w, h) = img.dimensions();
    let mut im = VecImg::from(img);

    for x in f.size / 2..w as usize - f.size / 2 {
        for y in f.size / 2..h as usize - f.size / 2 {
            im.apply_filter(x, y, f);
        }
    }

    let new_img: Vec<u8> = im.pixels.into_iter().map(|p| (p * 256.) as u8).collect();
    let imgbuf: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(w, h, new_img).expect("invalid vec");

    let out_name = format!(
        "dithered/{}_{}_{:.2}.png",
        filename.display(),
        f.name,
        im.threshold
    );
    println!("[+] Saving '{}'", out_name);
    imgbuf.save(out_name).expect("Could not save file");
}

fn main() {
    for filedir in std::fs::read_dir("imgs").expect("Wrong directory") {
        let file = filedir.expect("File not found").path();
        for alg in &[ATKINSON, STUCKI] {
            dither_file(&file, alg);
        }
        // dither_file(&file, &FS);
    }
}
