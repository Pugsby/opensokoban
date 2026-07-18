use macroquad::prelude::*;

pub fn sprite_rect(x: usize, y: usize) -> Rect {
    Rect::new(x as f32 * 64.0, y as f32 * 64.0, 64.0, 64.0)
}

pub fn draw_sprite_scaled(
    texture: &Texture2D,
    sprite: (usize, usize),
    x: f32,
    y: f32,
    scaled_tile_size: f32,
) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(scaled_tile_size, scaled_tile_size)),
            source: Some(sprite_rect(sprite.0, sprite.1)),
            ..Default::default()
        },
    );
}

pub fn get_player_sprite(direction: (i32, i32), color: usize) -> (usize, usize) {
    let ax = (color % 2) * 2;
    let ay = (color / 2) * 2;

    match direction {
        (1, 0) => (0 + ax, 2 + ay),
        (0, 1) => (1 + ax, 2 + ay),
        (0, -1) => (0 + ax, 3 + ay),
        (-1, 0) => (1 + ax, 3 + ay),
        _ => (0 + ax, 2 + ay),
    }
}
