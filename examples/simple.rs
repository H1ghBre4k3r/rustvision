use rustvision::{color::Color, image::Image, ppm::PNM, rgb, save_ppm};

fn main() {
    let mut img = Image::new(400, 400);
    img.fill(&rgb!(255, 0, 0));
    save_ppm!("assets/simple2.ppm", img);
}
