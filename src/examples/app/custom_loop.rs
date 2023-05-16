//! This example demonstrates you can create a custom runner (to update an app manually). It reads
//! 这个例子演示了如何创建一个自定义的运行器（用于手动更新应用程序）读取命令行输入并且打印它
//! lines from stdin and prints them from within the ecs.

use bevy::prelude::*;
use std::io;

#[derive(Resource)]
struct Input(String);

  fn my_runner(mut app: App) {
    println!("Type stuff into the console");
    for line in io::stdin().lines() {
        {
            let mut input = app.world.resource_mut::<Input>();
            input.0 = line.unwrap();
        }
        app.update();
    }
}

fn print_system(input: Res<Input>) {
    println!("You typed: {}", input.0);
}

pub fn custom_loop_main() {
    App::new()
    //将一个[Resource]插入到当前的[App]中，并覆盖之前添加的相同类型的[Resource]。
        .insert_resource(Input(String::new()))
        .set_runner(my_runner)
        .add_system(print_system)
        .run();
}