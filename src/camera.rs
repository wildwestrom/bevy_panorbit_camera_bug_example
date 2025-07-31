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
	commands.spawn((
		Transform::from_translation(Vec3::new(0., 1., 1.).normalize() * 50.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		Projection::from(PerspectiveProjection::default()),
		Camera::default(),
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
}
