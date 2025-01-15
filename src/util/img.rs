use image::{
    imageops::{flip_horizontal, flip_vertical, resize, rotate90, FilterType},
    Pixel,
};
use imageproc::{
    definitions::{Clamp, Image},
    filter::gaussian_blur_f32,
};

use super::ImageAction;

pub(crate) trait ImageHelper: Sized {
    fn proc(&self, act: &ImageAction) -> Self;
}

impl<P> ImageHelper for Image<P>
where
    P: Pixel + Send + Sync + 'static,
    <P as Pixel>::Subpixel: Send + Sync + Into<f32> + Clamp<f32>,
{
    fn proc(&self, act: &ImageAction) -> Self {
        match act {
            ImageAction::Blur(blur) => gaussian_blur_f32(self, *blur),
            ImageAction::Rotate => rotate90(self),
            ImageAction::Resize(ratio) => resize(
                self,
                (self.width() as f32 * ratio) as u32,
                (self.height() as f32 * ratio) as u32,
                FilterType::Lanczos3,
            ),
            ImageAction::FlipV => flip_vertical(self),
            ImageAction::FlipH => flip_horizontal(self),
        }
    }
}
