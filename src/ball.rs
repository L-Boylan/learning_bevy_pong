use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn draw_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
}