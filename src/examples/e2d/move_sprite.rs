//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

pub fn move_sprite_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    //SpriteBundle是Unity ECS（Entity Component System）框架中的一个组件包，用于在2D场景中创建和管理精灵。它包含了一些组件，如Sprite、SpriteRenderer、LocalToWorld、Translation等，可以用于控制精灵的位置、大小、旋转、渲染目标等属性。使用SpriteBundle可以方便地创建和管理2D场景中的精灵，同时也可以与其他组件和系统进行集成，实现更复杂的功能和效果。
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
/// 通过根据自上一帧以来经过的时间来改变其位置，来实现精灵的动画效果。
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}