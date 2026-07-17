#![allow(non_snake_case)]

use macroquad::prelude::*;

const TILE_SIZE: f32 = 48.0;

const FLOOR: u8 = 0;
const WALL: u8 = 1;
const GOAL: u8 = 2;
const CRATE: u8 = 3;
const CRATE_GOAL: u8 = 4;
const PLAYER: u8 = 5;
const PLAYER_GOAL: u8 = 6;

const MAX_LEVEL: usize = 60;

fn conf() -> Conf {
    let icon_16 = include_bytes!("../icons/16.png");
    let icon_32 = include_bytes!("../icons/32.png");
    let icon_64 = include_bytes!("../icons/64.png");

    let mut small_icon = [0; 16 * 16 * 4];
    let mut medium_icon = [0; 32 * 32 * 4];
    let mut big_icon = [0; 64 * 64 * 4];

    if let Ok(img) = image::load_from_memory(icon_16) {
        let pixels = img.to_rgba8().into_raw();
        if pixels.len() == small_icon.len() {
            small_icon.copy_from_slice(&pixels);
        }
    }

    if let Ok(img) = image::load_from_memory(icon_32) {
        let pixels = img.to_rgba8().into_raw();
        if pixels.len() == medium_icon.len() {
            medium_icon.copy_from_slice(&pixels);
        }
    }

    if let Ok(img) = image::load_from_memory(icon_64) {
        let pixels = img.to_rgba8().into_raw();
        if pixels.len() == big_icon.len() {
            big_icon.copy_from_slice(&pixels);
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

fn readBits(bytes: &[u8], bitCursor: &mut usize) -> Option<u8> {
    if *bitCursor + 3 > bytes.len() * 8 {
        return None;
    }

    let mut value = 0;

    for _ in 0..3 {
        let byte = bytes[*bitCursor / 8];
        let bit = (byte >> (7 - (*bitCursor % 8))) & 1;
        value = (value << 1) | bit;
        *bitCursor += 1;
    }

    Some(value)
}

async fn loadMap(
    id: usize
) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let bytes = include_bytes!("../levels.bin");

    let mut byteCursor = 0;
    let mut levelIndex = 0;

    while byteCursor + 2 <= bytes.len() {
        let width = bytes[byteCursor] as usize;
        let height = bytes[byteCursor + 1] as usize;
        byteCursor += 2;

        let mut map = vec![vec![FLOOR; width]; height];
        let mut player = (0, 0);
        let mut bitCursor = byteCursor * 8;

        for y in 0..height {
            for x in 0..width {
                let tile = readBits(bytes, &mut bitCursor).unwrap();

                map[y][x] = match tile {
                    1 => WALL,
                    2 => FLOOR,
                    3 => GOAL,
                    4 => GOAL,
                    5 => CRATE,
                    6 => {
                        player = (x, y);
                        PLAYER
                    }
                    _ => FLOOR,
                };
            }
        }

        byteCursor = (bitCursor + 7) / 8;

        if levelIndex == id {
            return (map, (width, height), player);
        }

        if byteCursor < bytes.len() && bytes[byteCursor] == 0 {
            byteCursor += 1;
        }

        levelIndex += 1;
    }

    (Vec::new(), (0, 0), (0, 0))
}

fn spriteRect(x: usize, y: usize) -> Rect {
    Rect::new(
        x as f32 * 64.0,
        y as f32 * 64.0,
        64.0,
        64.0
    )
}

fn drawSpriteScaled(
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
            source: Some(spriteRect(sprite.0, sprite.1)),
            ..Default::default()
        },
    );
}

fn getPlayerSprite(
    direction: (i32, i32)
) -> (usize, usize) {
    match direction {
        (1, 0) => (0, 2),
        (0, 1) => (1, 2),
        (0, -1) => (0, 3),
        (-1, 0) => (1, 3),
        _ => (0, 2),
    }
}

fn walkable(tile: u8) -> bool {
    tile == FLOOR || tile == GOAL
}

fn movePlayer(
    map: &mut Vec<Vec<u8>>,
    player: &mut (usize, usize),
    direction: (i32, i32)
) -> bool {
    let nx = player.0 as i32 + direction.0;
    let ny = player.1 as i32 + direction.1;

    if nx < 0 || ny < 0 {
        return false;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    if ny >= map.len() || nx >= map[0].len() {
        return false;
    }

    let target = map[ny][nx];

    if walkable(target) {
        let old = map[player.1][player.0];

        map[player.1][player.0] = if old == PLAYER_GOAL {
            GOAL
        } else {
            FLOOR
        };

        map[ny][nx] = if target == GOAL {
            PLAYER_GOAL
        } else {
            PLAYER
        };

        *player = (nx, ny);
        return true;
    }

    if target == CRATE || target == CRATE_GOAL {
        let bx = nx as i32 + direction.0;
        let by = ny as i32 + direction.1;

        if bx < 0 || by < 0 {
            return false;
        }

        let bx = bx as usize;
        let by = by as usize;

        let behind = map[by][bx];

        if walkable(behind) {
            map[by][bx] = if behind == GOAL {
                CRATE_GOAL
            } else {
                CRATE
            };

            map[ny][nx] = if target == CRATE_GOAL {
                PLAYER_GOAL
            } else {
                PLAYER
            };

            let old = map[player.1][player.0];

            map[player.1][player.0] = if old == PLAYER_GOAL {
                GOAL
            } else {
                FLOOR
            };

            *player = (nx, ny);
            return true;
        }
    }

    false
}

fn checkWin(
    map: &Vec<Vec<u8>>
) -> bool {
    for row in map {
        for tile in row {
            if *tile == CRATE {
                return false;
            }
        }
    }
    true
}

#[derive(Clone)]
struct GameState {
    map: Vec<Vec<u8>>,
    player: (usize, usize),
    direction: (i32, i32),
}

#[macroquad::main(conf)]
async fn main() {
    let asset_bytes = include_bytes!("../assets.png");
    let texture = Texture2D::from_file_with_format(asset_bytes, None);
    texture.set_filter(FilterMode::Nearest);

    let mut currentLevel = 1usize;

    let mut level = loadMap(currentLevel - 1).await;

    let mut map = level.0;
    let mut size = level.1;
    let mut player = level.2;

    let mut playerDirection = (1, 0);

    let mut history: Vec<GameState> = Vec::new();

    let mut show_credits = false;
    let mut show_go_to_level = false;
    let mut level_input_buffer = String::new();

    loop {
        let mut level_to_load: Option<usize> = None;

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Game", |ui| {
                        if ui.button("Go To Level").clicked() {
                            show_go_to_level = true;
                            level_input_buffer = currentLevel.to_string();
                            ui.close_menu();
                        }
                        if ui.button("Credits").clicked() {
                            show_credits = true;
                            ui.close_menu();
                        }
                        if ui.button("Source Code").clicked() {
                            let _ = open::that("https://github.com/Pugsby/opensokoban"); 
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });

            if show_credits {
                egui::Window::new("Credits")
                    .resizable(false)
                    .collapsible(false)
                    .show(egui_ctx, |ui| {
                        ui.label("Creator: Pugsby");
                        ui.label("Assets: Vellidragon");
                        ui.vertical_centered(|ui| {
                            if ui.button("Close").clicked() {
                                show_credits = false;
                            }
                        });
                    });
            }

            if show_go_to_level {
                egui::Window::new("Go To Level")
                    .resizable(false)
                    .collapsible(false)
                    .show(egui_ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Enter Level (1-60):");
                            let response = ui.text_edit_singleline(&mut level_input_buffer);
                            response.request_focus();
                        });

                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            if ui.button("Load").clicked() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                if let Ok(number) = level_input_buffer.parse::<usize>() {
                                    if number >= 1 && number <= MAX_LEVEL {
                                        level_to_load = Some(number);
                                        show_go_to_level = false;
                                    }
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                show_go_to_level = false;
                            }
                        });
                    });
            }
        });
        if let Some(target_lvl) = level_to_load {
            currentLevel = target_lvl;
            level = loadMap(currentLevel - 1).await;
            map = level.0;
            size = level.1;
            player = level.2;
            history.clear();
            playerDirection = (1, 0);
        }

        let mut wants_input = false;
        egui_macroquad::cfg(|ctx| {
            wants_input = ctx.wants_keyboard_input();
        });

        if !wants_input {
            if is_key_pressed(KeyCode::R) {
                level = loadMap(currentLevel - 1).await;
                map = level.0;
                player = level.2;
                history.clear();
                playerDirection = (1, 0);
            }

            if is_key_pressed(KeyCode::Backspace)
                || (is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Z))
            {
                if let Some(old) = history.pop() {
                    map = old.map;
                    player = old.player;
                    playerDirection = old.direction;
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
                    direction: playerDirection,
                });

                playerDirection = dir;

                if !movePlayer(&mut map, &mut player, dir) {
                    history.pop();
                }
            }

            if checkWin(&map) {
                if currentLevel < MAX_LEVEL {
                    currentLevel += 1;
                    level = loadMap(currentLevel - 1).await;
                    map = level.0;
                    size = level.1;
                    player = level.2;
                    history.clear();
                    playerDirection = (1, 0);
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
                drawSpriteScaled(&texture, (0, 1), bg_x, bg_y, scaled_tile);
                bg_x += scaled_tile;
            }
            bg_y += scaled_tile;
        }

        for y in 0..size.1 {
            for x in 0..size.0 {
                let render_x = vp_x + (x as f32 * scaled_tile);
                let render_y = vp_y + (y as f32 * scaled_tile);

                drawSpriteScaled(&texture, (0, 1), render_x, render_y, scaled_tile);

                match map[y][x] {
                    WALL => drawSpriteScaled(&texture, (2, 0), render_x, render_y, scaled_tile),
                    GOAL => drawSpriteScaled(&texture, (1, 1), render_x, render_y, scaled_tile),
                    CRATE => drawSpriteScaled(&texture, (0, 0), render_x, render_y, scaled_tile),
                    CRATE_GOAL => drawSpriteScaled(&texture, (1, 0), render_x, render_y, scaled_tile),
                    PLAYER => drawSpriteScaled(&texture, getPlayerSprite(playerDirection), render_x, render_y, scaled_tile),
                    PLAYER_GOAL => {
                        drawSpriteScaled(&texture, (1, 1), render_x, render_y, scaled_tile);
                        drawSpriteScaled(&texture, getPlayerSprite(playerDirection), render_x, render_y, scaled_tile);
                    }
                    _ => {}
                }
            }
        }

        egui_macroquad::draw();

        next_frame().await;
    }
}