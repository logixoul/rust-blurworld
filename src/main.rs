use macroquad::{prelude::*};
use ::rand::Rng;

fn blur(in_img:&Image) -> Image{
    let mut out_img = Image::gen_image_color(in_img.width, in_img.height, Color::from_hex(0x808080));
    let w = out_img.width as i32;
    let h = out_img.height as i32;
*
    for x in 0i32..w {
            for y in 0i32..h {
                let mut sum: f32 = 0.;
                for i in -1i32..=1i32 {
                    for j in -1i32..=1i32 {
                        let nb_x = x+i;
                        let nb_y= y+j;
                        let nb_x = (nb_x+w)%w;
                        let nb_y = (nb_y+h)%h;
                        sum += in_img.get_pixel(nb_x as u32, nb_y as u32).r;
                    }
                }
                let sum: f32 = sum/9.;
                out_img.set_pixel(x as u32, y as u32, Color::new(sum, sum, sum, 1.0));
            }
        }
    return out_img;
}

fn increase_contrast(in_img:&Image) -> Image{
    let mut out_img = Image::gen_image_color(in_img.width, in_img.height, Color::from_hex(0x808080));
    let w = out_img.width as i32;
    let h = out_img.height as i32;

    for y in 0i32..h {
        for x in 0i32..w {
                let mut f = in_img.get_pixel(x as u32, y as u32).r;
                f = clamp(f, 0., 1.);
                //f = if f < 0.5 { 0. } else { 1. };
                f = 3.*f*f-2.*f*f*f;
                out_img.set_pixel(x as u32, y as u32, Color::new(f, f, f, 1.0));
            }
        }
    return out_img;
}

fn make_random_image() -> Image {
    let mut img = Image::gen_image_color(500, 500, Color::from_hex(0x808080));
    let mut sum=0.;
    let mut rng = ::rand::rng();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let n : f32 = rng.random();
            sum+=n;
            img.set_pixel(x as u32, y as u32, Color::new(n, n, n, 1.));
        }
    }
    println!("{}", sum/(500.*500.));
    return img;
}

#[macroquad::main("Texture")]
async fn main() {
    let mut img : Image = make_random_image();
    loop {
        img = blur(&img);
        img = increase_contrast(&img);
        let tex: Texture2D = Texture2D::from_image(&img);
        clear_background(LIGHTGRAY);
        draw_texture(&tex, 0., 0., WHITE);
        next_frame().await
    }
}