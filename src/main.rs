use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};
use bevy_pixels::prelude::*;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const SCALE_FACTOR: f32 = 2.0;

const LETTER_H: [u8; 64] = [
    0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_e: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_l: [u8; 64] = [
    0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_o: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0,
    0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_W: [u8; 64] = [
    1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_r: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0,
    0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_d: [u8; 64] = [
    0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0,
    1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const LETTER_EXCL: [u8; 64] = [
    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[derive(Bundle, Debug)]
struct LetterBundle {
    position: Position,
    size: Size,
    color: Color,
}

#[derive(Component, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Component, Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy Pixels".to_string(),
                resolution: WindowResolution::new(
                    SCREEN_WIDTH as f32 * SCALE_FACTOR,
                    SCREEN_HEIGHT as f32 * SCALE_FACTOR,
                ),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            primary_window: Some(PixelsOptions {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                scale_factor: SCALE_FACTOR,
                ..default()
            }),
        })
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_systems(
            (draw_background, draw_letter)
                .chain()
                .in_set(PixelsSet::Draw),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let letter = LetterBundle {
        position: Position { x: 8, y: 8 },
        size: Size {
            width: 8,
            height: 8,
        },
        color: Color(0xff, 0xff, 0x00, 0xff),
    };
    commands.spawn(letter);
}

fn draw_background(mut wrapper_query: Query<&mut PixelsWrapper>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();

    frame.copy_from_slice(&[0x00, 0x00, 0xff, 0xff].repeat(frame.len() / 4));
}

fn draw_letter(
    mut wrapper_query: Query<(&mut PixelsWrapper, &PixelsOptions)>,
    query: Query<(&Position, &Size, &Color)>,
) {
    let Ok((mut wrapper, options)) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    let frame_width_bytes = (options.width * 4) as usize;

    for (position, size, color) in &query {
        for x in 0..size.width {
            for y in 0..size.height {
                if LETTER_H[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_e[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 8) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_l[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 16) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_l[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 24) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_o[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 32) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_W[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 48) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_o[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 56) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_r[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 64) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_l[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 72) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_d[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 80) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
                if LETTER_EXCL[(x + y * size.width) as usize] == 1 {
                    let x_offset = ((x + position.x + 88) * 4) as usize;
                    let idx = x_offset + (y + position.y) as usize * frame_width_bytes;
                    frame[idx..idx + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
                }
            }
        }
    }
}
