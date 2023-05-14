//! Displays a single [`Sprite`], created from an image.
//! 显示一个[Sprite]，该[Sprite]是从一张图片创建的。

use bevy::prelude::*;
 
pub fn sprite_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system( setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
  
    //在计算机图形学中，Sprite（精灵）是指一个图像或动画，可以在屏幕上移动或与其他图像进行交互。在游戏开发中，Sprite通常用于表示游戏中的角色、道具、背景等元素。Sprite通常是由一个或多个图像帧组成的动画，可以在屏幕上播放，以模拟运动或其他动态效果。在编程中，Sprite通常是作为一个对象来实现的，可以包含位置、大小、速度、动画帧等属性。
     //在计算机科学中，Bundle（捆绑包）通常指一组相关的文件或数据，它们被打包在一起以便于传输、存储或处理。Bundle可以包含任何类型的文件或数据，例如应用程序、库、配置文件、图像、音频、视频等。在移动应用程序开发中，Bundle通常用于在应用程序之间传递数据，例如从一个Activity传递数据到另一个Activity。在操作系统中，Bundle也可以用于打包和传递系统级别的数据，例如在Android中，Intent对象可以包含一个Bundle，用于在应用程序之间传递数据。Bundle的使用可以使数据传输更加高效和可靠，同时也可以提高数据的安全性和可维护性。
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        ..default()
    });
}