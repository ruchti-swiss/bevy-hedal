use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;

#[derive(Component)]
struct Cube;

#[derive(Resource, Default)]
struct LastTouchPos(Option<Vec2>);

#[derive(Resource)]
struct RotationDirection(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
        )        
        .insert_resource(RotationDirection(1.0))
        .insert_resource(LastTouchPos(None))
        .add_systems(Startup, setup)
        .add_systems(Update,(auto_rotate, keyboard_control, touch_control))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("Hedalicon_white.png")), // <-- dein echter Name
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true, // <-- sonst von hinten unsichtbar
        cull_mode: None,    // <-- sonst wird Rückseite weggeschnitten
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(2.0, 2.0))),
        MeshMaterial3d(mat),
        Transform::default(),
        Cube, // <-- DAS ist wichtig für die Rotation!
    ));
}

// automatische Rotation
fn auto_rotate(
    time: Res<Time>,
    direction: Res<RotationDirection>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(
            direction.0 * time.delta_secs()
        );
    }
}

// Tastatur
fn keyboard_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut direction: ResMut<RotationDirection>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    for mut transform in &mut query {

        // Drehrichtung ändern
        if keyboard.just_pressed(KeyCode::ArrowLeft) {
            direction.0 = -1.0;
        }

        if keyboard.just_pressed(KeyCode::ArrowRight) {
            direction.0 = 1.0;
        }

        // Kippen
        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.rotate_x(
                2.0 * time.delta_secs()
            );
        }

        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.rotate_x(
                -2.0 * time.delta_secs()
            );
        }
    }
}

// Touch-Steuerung
fn touch_control(
    touches: Res<Touches>,
    mut last_touch: ResMut<LastTouchPos>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    let sensitivity = 0.01;

    if let Some(touch) = touches.iter().next() {
        let pos = touch.position();

        if let Some(prev) = last_touch.0 {
            let delta = pos - prev;

            for mut transform in &mut query {

                // links/rechts ziehen
                transform.rotate_y(
                    -delta.x * sensitivity
                );

                // hoch/runter ziehen
                transform.rotate_x(
                    -delta.y * sensitivity
                );
            }
        }

        last_touch.0 = Some(pos);
    } else {
        last_touch.0 = None;
    }

}
