use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;

#[derive(Component)]
struct Cube;

#[derive(Resource, Default)]
struct LastTouchPos(Option<Vec2>);

#[derive(Resource, Default)]
struct MouseWorldPos(Vec2);

#[derive(Resource)]
struct RotationDirection(f32);

#[derive(Component)]
struct TechNode {
    base_pos: Vec2,
    phase: f32,
    speed: f32,
    radius: f32,
}


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
        // Dunkler, violett-getönter Tech-Hintergrund (#14111A)
        .insert_resource(ClearColor(Color::srgb_u8(20, 17, 26)))
        .add_systems(Startup, setup_background)
        .add_systems(Update, (animate_nodes, draw_network))
        .insert_resource(MouseWorldPos::default())
        .add_systems(Update, update_mouse_position)
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
    mut direction: ResMut<RotationDirection>,
    mut last_touch: ResMut<LastTouchPos>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    let sensitivity = 0.01;

    if let Some(touch) = touches.iter().next() {
        let pos = touch.position();

        if let Some(prev) = last_touch.0 {
            let delta = pos - prev;
        
            // Drehrichtung wie ArrowLeft / ArrowRight
            if delta.x < 0.0 {
                direction.0 = -1.0;
            } else if delta.x > 0.0 {
                direction.0 = 1.0;
            }
        
            for mut transform in &mut query {
                transform.rotate_y(-delta.x * sensitivity);
                transform.rotate_x(-delta.y * sensitivity);
            }
        }

        last_touch.0 = Some(pos);
    } else {
        last_touch.0 = None;
    }

}


fn setup_background(
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let Ok(window) = windows.single() else {
        return;
    };

    let spacing_x = 140.0;
    let spacing_y = 120.0;

    let width = window.width();
    let height = window.height();

    let cols = (width / spacing_x).ceil() as usize + 4;
    let rows = (height / spacing_y).ceil() as usize + 4;

    let start_x = -width * 0.5;
    let start_y = -height * 0.5;

    for row in 0..rows {
        for col in 0..cols {
            let index = (row * cols + col) as f32;

            let offset_x = if row % 2 == 0 {
                0.0
            } else {
                spacing_x * 0.5
            };

            let x = start_x + col as f32 * spacing_x + offset_x;
            let y = start_y + row as f32 * spacing_y;

            commands.spawn((
                TechNode {
                    base_pos: Vec2::new(x, y),
                    phase: index * 0.7,
                    speed: 0.6 + (index % 5.0) * 0.15,
                    radius: 25.0 + (index % 3.0) * 10.0,
                },
                Transform::from_xyz(x, y, -10.0),
            ));
        }
    }
}

fn update_mouse_position(
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mouse_pos: ResMut<MouseWorldPos>,
) {
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) =
            camera.viewport_to_world_2d(camera_transform, cursor_pos)
        {
            mouse_pos.0 = world_pos;
        }
    }
}

fn animate_nodes(
    time: Res<Time>,
    mouse: Res<MouseWorldPos>,
    mut query: Query<(&TechNode, &mut Transform)>,
) {
    let t = time.elapsed_secs();

    for (node, mut transform) in &mut query {
        let angle = t * node.speed + node.phase;

        let mut pos = Vec2::new(
            node.base_pos.x + angle.cos() * node.radius,
            node.base_pos.y + (angle * 0.8).sin() * node.radius,
        );

        let dir = mouse.0 - pos;
        let dist = dir.length();

        if dist < 250.0 {
            let strength = (250.0 - dist) / 250.0;
            pos += dir.normalize_or_zero() * strength * 50.0;
        }

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

fn draw_network(mut gizmos: Gizmos, query: Query<&Transform, With<TechNode>>) {
    let positions: Vec<Vec2> = query
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    let max_dist = 190.0;

    for i in 0..positions.len() {
        // Zeichnet kleine leuchtende Punkte im Logo-Violett (#9E4795)
        gizmos.circle_2d(positions[i], 2.5, Color::srgba_u8(158, 71, 149, 180));

        for j in (i + 1)..positions.len() {
            let dist = positions[i].distance(positions[j]);
            if dist < max_dist {
                // Je näher die Punkte, desto sichtbarer die Verbindungslinie
                let alpha = ((1.0 - dist / max_dist) * 110.0) as u8;
                gizmos.line_2d(
                    positions[i],
                    positions[j],
                    Color::srgba_u8(138, 59, 130, alpha),
                );
            }
        }
    }
}
