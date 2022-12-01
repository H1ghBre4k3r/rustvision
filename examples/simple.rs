use rustvision::{image::Image, ppm::PNM, rgb, save_pnm_p3, save_pnm_p6, shapes::Rectangle, vec2};

fn main() {
    let mut img = Image::new(400, 400);
    img.fill_with(&rgb!(0, 255, 0));

    let rect = Rectangle::new(vec2![50.0, 40.0], 100, 70, rgb!(255, 0, 0));
    img.draw(&rect);

    save_pnm_p3!("assets/simple2.ppm", img);
    save_pnm_p6!("assets/simple3.ppm", img);
}
