use rustvision::{
    image::Image,
    ppm::PNM,
    rgb, save_pnm_p3, save_pnm_p6,
    shapes::{Line, Rectangle},
    vec2,
};

fn main() {
    let mut img = Image::new(400, 400);
    img.fill_with(&rgb!(0, 255, 0));

    let rect = Rectangle::new(vec2![50.0, 40.0], 100, 70, rgb!(255, 0, 0));
    let line = Line::new(vec2![200.0, 200.0], vec2![250.0, 230.0]).with_color(rgb!(0, 0, 255));
    img.draw(&rect);
    img.draw(&line);

    save_pnm_p3!("assets/simple2.ppm", img);
    save_pnm_p6!("assets/simple3.ppm", img);

    img.fill_with(&rgb!(0, 0, 0));
    save_pnm_p3!("assets/black.ppm", img);

    img.fill_with(&rgb!(255, 255, 255));
    save_pnm_p3!("assets/white.ppm", img);
}
