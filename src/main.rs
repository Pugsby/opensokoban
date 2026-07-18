#![allow(non_snake_case)]

mod config;
mod game;
mod level;
mod render;
mod ui;

use config::{conf, MAX_LEVEL, TILE_SIZE};
use game::{check_win, move_player, GameState};
use level::load_map;
use macroquad::prelude::*;
use render::{draw_sprite_scaled, get_player_sprite};
use ui::{update_ui, UiState};

#[macroquad::main(conf)]
async fn main() {
    let asset_bytes = include_bytes!("../assets.png");
    let texture = Texture2D::from_file_with_format(asset_bytes, None);
    texture.set_filter(FilterMode::Nearest);

    macroquad::rand::srand((get_time() * 1_000_000.0) as u64);
    let player_color = macroquad::rand::gen_range(0usize, 4usize);

    let mut current_level = 1usize;

    let mut level = load_map(current_level - 1).await;

    let mut map = level.0;
    let mut size = level.1;
    let mut player = level.2;

    let mut player_direction = (1, 0);
    let mut history: Vec<GameState> = Vec::new();
    let mut ui_state = UiState::new(current_level);

    loop {
        if let Some(target_level) = update_ui(&mut ui_state, current_level) {
            current_level = target_level;
            level = load_map(current_level - 1).await;
            map = level.0;
            size = level.1;
            player = level.2;
            history.clear();
            player_direction = (1, 0);
        }

        let mut wants_input = false;
        egui_macroquad::cfg(|ctx| {
            wants_input = ctx.wants_keyboard_input();
        });

        if !wants_input {
            if is_key_pressed(KeyCode::R) {
                level = load_map(current_level - 1).await;
                map = level.0;
                player = level.2;
                history.clear();
                player_direction = (1, 0);
            }

            if is_key_pressed(KeyCode::Backspace)
                || (is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Z))
            {
                if let Some(old) = history.pop() {
                    map = old.map;
                    player = old.player;
                    player_direction = old.direction;
                }
            }

            let direction = if is_key_pressed(KeyCode::Left) {
                Some((-1, 0))
            } else if is_key_pressed(KeyCode::Right) {
                Some((1, 0))
            } else if is_key_pressed(KeyCode::Up) {
                Some((0, -1))
            } else if is_key_pressed(KeyCode::Down) {
                Some((0, 1))
            } else {
                None
            };

            if let Some(dir) = direction {
                history.push(GameState {
                    map: map.clone(),
                    player,
                    direction: player_direction,
                });

                player_direction = dir;

                if !move_player(&mut map, &mut player, dir) {
                    history.pop();
                }
            }

            if check_win(&map) {
                if current_level < MAX_LEVEL {
                    current_level += 1;
                    level = load_map(current_level - 1).await;
                    map = level.0;
                    size = level.1;
                    player = level.2;
                    history.clear();
                    player_direction = (1, 0);
                }
            }
        }

        let base_w = size.0 as f32 * TILE_SIZE;
        let base_h = size.1 as f32 * TILE_SIZE;

        if screen_width() < base_w || screen_height() < base_h {
            request_new_screen_size(
                f32::max(screen_width(), base_w),
                f32::max(screen_height(), base_h),
            );
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        let scale = f32::floor(f32::min(screen_w / base_w, screen_h / base_h)).max(1.0);
        let scaled_tile = TILE_SIZE * scale;

        let vp_width = base_w * scale;
        let vp_height = base_h * scale;

        let vp_x = f32::floor((screen_w - vp_width) / 2.0);
        let vp_y = f32::floor((screen_h - vp_height) / 2.0);

        clear_background(BLACK);

        let start_x = ((vp_x % scaled_tile) - scaled_tile).floor();
        let start_y = ((vp_y % scaled_tile) - scaled_tile).floor();

        let mut bg_y = start_y;
        while bg_y < screen_h + scaled_tile {
            let mut bg_x = start_x;
            while bg_x < screen_w + scaled_tile {
                draw_sprite_scaled(&texture, (0, 1), bg_x, bg_y, scaled_tile);
                bg_x += scaled_tile;
            }
            bg_y += scaled_tile;
        }

        for y in 0..size.1 {
            for x in 0..size.0 {
                let render_x = vp_x + (x as f32 * scaled_tile);
                let render_y = vp_y + (y as f32 * scaled_tile);

                draw_sprite_scaled(&texture, (0, 1), render_x, render_y, scaled_tile);

                match map[y][x] {
                    config::WALL => draw_sprite_scaled(&texture, (2, 0), render_x, render_y, scaled_tile),
                    config::GOAL => draw_sprite_scaled(&texture, (1, 1), render_x, render_y, scaled_tile),
                    config::CRATE => draw_sprite_scaled(&texture, (0, 0), render_x, render_y, scaled_tile),
                    config::CRATE_GOAL => draw_sprite_scaled(&texture, (1, 0), render_x, render_y, scaled_tile),
                    config::PLAYER => draw_sprite_scaled(&texture, get_player_sprite(player_direction, player_color), render_x, render_y, scaled_tile),
                    config::PLAYER_GOAL => {
                        draw_sprite_scaled(&texture, (1, 1), render_x, render_y, scaled_tile);
                        draw_sprite_scaled(&texture, get_player_sprite(player_direction, player_color), render_x, render_y, scaled_tile);
                    }
                    _ => {}
                }
            }
        }

        egui_macroquad::draw();

        next_frame().await;
    }
}
