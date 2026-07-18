use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 48.0;

pub const FLOOR: u8 = 0;
pub const WALL: u8 = 1;
pub const GOAL: u8 = 2;
pub const CRATE: u8 = 3;
pub const CRATE_GOAL: u8 = 4;
pub const PLAYER: u8 = 5;
pub const PLAYER_GOAL: u8 = 6;

pub const MAX_LEVEL: usize = 60;

pub fn conf() -> Conf {
    let icon_64 = include_bytes!("../icon.png");

    let mut small_icon = [0; 16 * 16 * 4];
    let mut medium_icon = [0; 32 * 32 * 4];
    let mut big_icon = [0; 64 * 64 * 4];

    if let Ok(img) = image::load_from_memory(icon_64) {
        let rgba = img.to_rgba8();

        let pixels_big = rgba.clone().into_raw();
        if pixels_big.len() == big_icon.len() {
            big_icon.copy_from_slice(&pixels_big);
        }

        let pixels32 = image::imageops::resize(&rgba, 32, 32, image::imageops::FilterType::Lanczos3).into_raw();
        if pixels32.len() == medium_icon.len() {
            medium_icon.copy_from_slice(&pixels32);
        }

        let pixels16 = image::imageops::resize(&rgba, 16, 16, image::imageops::FilterType::Lanczos3).into_raw();
        if pixels16.len() == small_icon.len() {
            small_icon.copy_from_slice(&pixels16);
        }
    }

    let mq_icon = macroquad::miniquad::conf::Icon {
        small: small_icon,
        medium: medium_icon,
        big: big_icon,
    };

    Conf {
        window_title: "Open Sokoban".to_string(),
        icon: Some(mq_icon),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}
