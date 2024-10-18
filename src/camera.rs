use bevy::prelude::*;

const CAMERA_START: Vec2 = Vec2::new(960., 450.);
pub const CAMERA_PLANAR_SPEED: f32 = 800.0;
pub const CAMERA_ZOOM_RATE: f32 = 1.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {},
            transform: Transform::from_translation(CAMERA_START.extend(0.0)),
            ..default()
        },
        CameraDirection {
            planar: Vec2::ZERO,
            height: 0.0,
        },
    ));
}

#[derive(Component)]
pub struct CameraDirection {
    pub planar: Vec2,
    pub height: f32,
}

fn camera_movement(
    mut query_camera: Query<
        (
            &mut OrthographicProjection,
            &mut Transform,
            &CameraDirection,
        ),
        With<Camera2d>,
    >,
    time: Res<Time>,
) {
    let (mut projection, mut transform, direction) = query_camera.single_mut();

    if direction.planar != Vec2::ZERO {
        transform.translation +=
            direction.planar.extend(0.0) * CAMERA_PLANAR_SPEED * time.delta_seconds();
    }

    if direction.height > 0.0 {
        projection.scale *= direction.height * CAMERA_ZOOM_RATE * time.delta_seconds();
    } else if direction.height < 0.0 {
        projection.scale /= direction.height.abs() * CAMERA_ZOOM_RATE * time.delta_seconds();
    }
}
