use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(PanOrbitCameraPlugin)
		.add_systems(Startup, setup)
		.add_systems(Startup, make_cube)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Transform::from_translation(Vec3::new(0., 1., 1.).normalize() * 50.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		Projection::from(PerspectiveProjection::default()),
		PanOrbitCamera::default(),
	));
}

fn make_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Mesh3d(meshes.add(Rectangle::from_size(Vec2::splat(10.0)))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::oklch(1.0, 1.0, 0.0)))),
		Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -90_f32.to_radians())),
	));
}
