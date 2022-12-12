use rustvision::{load_image, save_pnm_p3};

fn main() {
    let image = load_image!("assets/mona_lisa_small.ppm");

    save_pnm_p3!("assets/mona_lisa_small_2.ppm", image);
}
