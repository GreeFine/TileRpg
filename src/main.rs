use bevy::{prelude::*, window::WindowMode};

mod consts;
mod systems;

fn main() {
    let mut app = App::build();

    app.add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_resource(WindowDescriptor {
            title: "game".to_string(),
            vsync: true,
            resizable: false,
            height: 720.,
            width: 1280.,
            // mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin::default());

    systems::init(&mut app);

    app.add_startup_system(setup.system()).run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
