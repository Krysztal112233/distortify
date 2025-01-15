use rand::{thread_rng, Rng, RngCore};

pub(crate) mod img;
pub(crate) mod path;

#[derive(Debug, strum::Display)]
pub(crate) enum ImageAction {
    Blur(f32),
    Rotate,
    Resize(f32),
    FlipV,
    FlipH,
}

impl ImageAction {
    pub(crate) fn random() -> (ImageAction, usize) {
        let iter = thread_rng().gen_range(0..5usize);

        match thread_rng().next_u32() % 5 {
            0 => (Self::Blur(thread_rng().gen_range(0.8..1.2) as f32), iter),
            1 => (Self::Rotate, iter),
            2 => (Self::Resize(thread_rng().gen_range(0.8..1.1)), iter),
            3 => (Self::FlipV, iter),
            4 => (Self::FlipH, iter),
            _ => panic!(),
        }
    }
}
