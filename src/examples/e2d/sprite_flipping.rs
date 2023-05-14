//! Displays a single [`Sprite`], created from an image, but flipped on one axis.
//! 显示一个[Sprite]，该[Sprite]是从一张图片创建的，但在一个轴上翻转。

use bevy::prelude::*;

pub fn sprite_flipping_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(  setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        sprite: Sprite {
            // Flip the logo to the left
            flip_x: true,
            // And don't flip it upside-down ( the default )
            flip_y: true,
            ..default()
        },
        ..default()
    });
}