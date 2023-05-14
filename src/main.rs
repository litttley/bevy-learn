use examples::{
    e2d::{
        bloom_2d::bloom_2d_main, e2d_shapes::e2d_shapes_main, mesh2d::mesh2d_main,
        mesh2d_manual::mesh2d_manual_main,
        mesh2d_vertex_color_texture::mesh2d_vertex_color_texture_main,
        move_sprite::move_sprite_main, pixel_perfect::pixel_perfect_main, rotation::rotation_main, sprite::sprite_main, sprite_flipping::sprite_flipping_main, sprite_sheet::sprite_sheet_main, text2d::text2d_main, texture_atlas::texture_atlas_main, transparency_2d::transparency_2d_main,
    },
    hello_world::hello_wrold_main,
    ui::{button::button_main, font_atlas_debug::font_atlas_debug_main, relative_cursor_position::relative_cursor_position_main, text::text_main, text_debug::text_debug_main, text_layout::text_layout_main, transparency_ui::transparency_ui_main, ui::ui_main}, e3d::{e3d_scene::e3d_scene_main, e3d_shapes::e3d_shapes_main},
};

pub mod examples;

fn main() {
    // hello_wrold_main()
    /********2d-start ************************/
    // e2d_shapes_main()
    // bloom_2d_main()
    // mesh2d_main()
    // mesh2d_manual_main()
    // mesh2d_vertex_color_texture_main()
    // move_sprite_main()
    // pixel_perfect_main()
    // rotation_main()
    // sprite_main()
    // sprite_flipping_main()
    // sprite_sheet_main()
    // text2d_main()
    // texture_atlas_main()
    // transparency_2d_main()

    /********2d-end************************/

    /********3d-start************************/
    // e3d_scene_main()
    // e3d_shapes_main()
   
    /********3d-end************************/
    /********ui-start ************************/
    // button_main()
    // font_atlas_debug_main()
     // relative_cursor_position_main()
    // text_main()
    // text_debug_main()
    // text_layout_main()
    // transparency_ui_main()
    
    ui_main()
    /********ui-end************************/
}
