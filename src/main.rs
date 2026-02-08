use macroquad::{prelude::*, texture};
use ::rand::Rng;
use crate::array_2d::Array2D;
//use macroquad::{get_context, get_quad_context};

mod array_2d;

type Cell = f32;
type CellArray = Array2D<Cell>;

fn blur(in_img:&CellArray) -> CellArray {
    let mut out_img = CellArray::new(in_img.width, in_img.height);
    let w = out_img.width as i32;
    let h = out_img.height as i32;

    for x in 0i32..w {
            for y in 0i32..h {
                let mut sum: f32 = 0.;
                for i in -1i32..=1i32 {
                    for j in -1i32..=1i32 {
                        let nb_x = x+i;
                        let nb_y= y+j;
                        let nb_x = (nb_x+w)%w;
                        let nb_y = (nb_y+h)%h;
                        sum += in_img.get(nb_x, nb_y);
                    }
                }
                let sum: f32 = sum/9.;
                out_img.set(x, y, sum);
            }
        }
    return out_img;
}

fn increase_contrast(in_img:&CellArray) -> CellArray{
    let mut out_img = CellArray::new(in_img.width, in_img.height);
    let w = out_img.width as i32;
    let h = out_img.height as i32;

    for y in 0i32..h {
        for x in 0i32..w {
                let mut f = in_img.get(x, y);
                f = clamp(f, 0., 1.);
                //f = if f < 0.5 { 0. } else { 1. };
                f = 3.*f*f-2.*f*f*f;
                out_img.set(x, y, f);
            }
        }
    return out_img;
}

fn make_random_image() -> CellArray {
    let mut img = CellArray::new(500, 500);
    let mut sum=0.;
    let mut rng = ::rand::rng();
    for y in 0..img.height {
        for x in 0..img.width {
            let n : f32 = rng.random();
            sum += n;
            img.set(x, y, n);
        }
    }
    let area = (img.width * img.height) as f32;
    println!("{}", sum/area);
    return img;
}

fn texture_from_cell_array(data: &CellArray) -> Texture2D {
    let mut data_8u = Image::gen_image_color(data.width as u16, data.height as u16, Color::new(0., 0., 0., 0.));
    for y in 0..data.height {
        for x in 0..data.width {
            let f = data.get(x, y);
            data_8u.set_pixel(x as u32, y as u32, Color::new(f, f, f, 1.0));
        }
    }
    return Texture2D::from_image(&data_8u);
}

#[macroquad::main("Texture")]
async fn main() {
    let mut img : CellArray = make_random_image();
    loop {
        img = blur(&img);
        img = increase_contrast(&img);
        //let imgBytes = vec!(img.width * img.height * 4, 0);

        let tex: Texture2D = texture_from_cell_array(&img);
        clear_background(LIGHTGRAY);
        draw_texture(&tex, 0., 0., WHITE);
        next_frame().await
    }
}