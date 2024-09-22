use image::ImageBuffer;

pub type RGBImage = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub fn transpose_image(img: &RGBImage) -> RGBImage {
    let mut img_transposed = RGBImage::new(img.height(), img.width());

    for (x, y, pixel) in img_transposed.enumerate_pixels_mut() {
        *pixel = *img.get_pixel(y, x);
    }

    img_transposed
}
