use rustvision::{
    image::Image,
    ppm::PNM,
    rgb, save_pnm_p3, save_pnm_p6,
    shapes::{Line, Polygon, Rectangle},
    vec2,
};

fn main() {
    let mut img = Image::new(400, 400);

    let rect = Rectangle::new(vec2![50.0, 40.0], 100, 70, rgb!(255, 0, 0));
    let line = Line::new(vec2![200.0, 200.0], vec2![250.0, 230.0]).with_color(rgb!(0, 0, 255));

    let mut polygon = Polygon::from_points(vec![
        vec2![20.0, 250.0],
        vec2![50.0, 350.0],
        vec2![80.0, 280.0],
        vec2![110.0, 350.0],
        vec2![140.0, 250.0],
        vec2![110.0, 300.0],
        vec2![80.0, 250.0],
        vec2![50.0, 300.0],
    ]);
    polygon.set_color(rgb!(0, 255, 0));
    polygon.set_filled(true);

    let mut quat = Polygon::from_points(vec![
        vec2![200.0, 40.0],
        vec2![200.0, 100.0],
        vec2![300.0, 100.0],
        vec2![300.0, 40.0],
    ]);
    quat.set_color(rgb!(0, 255, 0));
    quat.set_filled(true);

    img.draw(&rect);
    img.draw(&line);
    img.draw(&polygon);
    img.draw(&quat);

    save_pnm_p3!("assets/simple2.ppm", img);
    save_pnm_p6!("assets/simple3.ppm", img);
}
