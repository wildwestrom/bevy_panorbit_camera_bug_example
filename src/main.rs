use bevy::prelude::*;

mod camera;
mod hud;

use crate::camera::CameraPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(CameraPlugin)
		.add_systems(Startup, make_cube)
		.run();
}

fn make_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(10.0)))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::oklch(1.0, 1.0, 0.0)))),
		Transform::default(),
	));
}
