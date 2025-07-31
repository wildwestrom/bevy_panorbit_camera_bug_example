use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(PanOrbitCameraPlugin)
			.add_plugins(crate::hud::CameraDebugHud)
			.add_systems(Startup, setup);
	}
}

fn setup(mut commands: Commands) {
	let (transform, perspective) = create_perspective_angled_state(50.0); // Just a random value to test its smooth

	commands.spawn((
		transform,
		Projection::from(perspective),
		//Camera3d::default(),
		Camera {
			order: 0,
			..default()
		},
		RenderLayers::layer(0),
		PanOrbitCamera::default(),
	));

	commands.spawn((
		DirectionalLight {
			illuminance: light_consts::lux::OVERCAST_DAY,
			shadows_enabled: true,
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 2.0, 0.0),
			rotation: Quat::from_rotation_x(-PI / 4.),
			..default()
		},
	));

	commands.spawn((
		Camera2d,
		Camera {
			order: 1,
			..default()
		},
		RenderLayers::layer(1),
	));
}

fn create_perspective_angled_state(size: f32) -> (Transform, PerspectiveProjection) {
	let fov = 60.0_f32.to_radians();
	// Desired camera position at 60deg FOV, looking from a diagonal angle
	let distance = dolly_zoom_distance(size, fov);
	let initial_angle = Vec3::new(0., 1., 1.);
	let angled_pos = initial_angle.normalize() * distance;
	let transform = Transform::from_translation(angled_pos).looking_at(Vec3::ZERO, Vec3::Y);
	let projection = PerspectiveProjection {
		fov: fov,
		far: 10000.0,
		..default()
	};
	(transform, projection)
}

fn dolly_zoom_distance(width: f32, fov: f32) -> f32 {
	width / (2.0 * (0.5 * fov).tan())
}
