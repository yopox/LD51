// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{App, Camera2dBundle, ClearColor, Color, Commands, Msaa, NonSend, Transform, WindowDescriptor};
use bevy::render::texture::ImageSettings;
use bevy::winit::WinitWindows;

use bevy_game::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Every 10 seconds".to_string(),
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(init)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    // let primary = windows.get_window(WindowId::primary()).unwrap();
    // let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    // if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
    //     let image = image.into_rgba8();
    //     let (width, height) = image.dimensions();
    //     let rgba = image.into_raw();
    //     let icon = Icon::from_rgba(rgba, width, height).unwrap();
    //     primary.set_window_icon(Some(icon));
    // };
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.25, 0.25, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}