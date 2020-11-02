use bevy::{prelude::*, app::AppExit};
use bevy::input::system::exit_on_esc_system;
// use bevy::window::WindowMode::Fullscreen;


fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Dr. Flocker".to_string(),
            width: 1024,
            height: 768,
            vsync: true,
            resizable: false,
            /*
            mode: Fullscreen {
                use_size: false
            },
            */
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .add_system(keyboard_input_system.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: ResMut<Events<AppExit>>) {

    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}

/// rotates the parent, which will result in the child also rotating
fn rotator_system(time: Res<Time>, mut query: Query<(&Rotator, &mut Transform)>) {
    for (_rotator, mut transform) in &mut query.iter() {
        transform.rotate(Quat::from_rotation_z(2.0 * time.delta_seconds));
        transform.translate(Vec3::new(10.0, 0.0, 0.0));
        transform.apply_scale(1.0 - (time.delta_seconds / 5.0));
    }
}

struct Rotator;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/sprites/smilechaser.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Rotator);
}

