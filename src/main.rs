use bevy::prelude::*;

#[derive(Component)]
struct Cube;

#[derive(Resource, Default)]
struct LastTouchPos(Option<Vec2>);

#[derive(Resource)]
struct RotationDirection(f32);

fn main() {
    App::new()
        .insert_resource(LastTouchPos::default())
        .insert_resource(RotationDirection(1.0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                auto_rotate,
                keyboard_control,
                touch_control,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 6.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 1.0))),
        Transform::default(),
        Cube,
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
