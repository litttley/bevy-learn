//! An example that shows how to handle drag and drop of files in an app.
//! 一个示例，演示如何在应用程序中处理文件的拖放操作

use bevy::prelude::*;

pub fn drag_and_drop_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(file_drag_and_drop_system)
        .run();
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.iter() {
        
        info!("{:?}", event);
    }
}