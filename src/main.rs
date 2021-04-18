use bevy::prelude::*;

mod consts;
mod systems;

fn main() {
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WindowDescriptor {
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

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);
}
