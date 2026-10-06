use image::Rgba;

pub struct Blender;

impl Blender {
    pub fn alpha(dst: Rgba<u8>, src: Rgba<u8>) -> Rgba<u8> {
        let a = src[3] as u32;
        let inv_a = 255 - a;

        Rgba([
            ((src[0] as u32 * a + dst[0] as u32 * inv_a) / 255) as u8,
            ((src[1] as u32 * a + dst[1] as u32 * inv_a) / 255) as u8,
            ((src[2] as u32 * a + dst[2] as u32 * inv_a) / 255) as u8,
            255,
        ])
    }
}
