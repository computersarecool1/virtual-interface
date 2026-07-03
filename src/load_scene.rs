use std::process::Command;

use bevy::{color::palettes::css::RED, prelude::*, sprite::Anchor};
use image::codecs::pnm::ArbitraryTuplType::RGB;

use crate::{Code, EnumCode};

pub struct loadPlugin;

impl Plugin for loadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load);        

    }
}
#[derive(Component)]
struct Load;
fn load(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut i = 0;

    let width = 0.45;
    let height = 0.45;
    let w = 35;
    let h = 30;
    for x in 1..w {     for y in 0..h { 

let entit= commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.,1.))),
        Transform::from_translation(Vec3 { x: 9. + y as f32 *width as f32*50 as f32, y: -310. + x as f32 *height as f32*50 as f32, z: -32. }).with_scale(Vec3 { x: width, y: height, z: 2.}),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba::rgb(0.1*x as f32, 1.0, 0.0)))),
        Load,Code{ s: EnumCode::MyKey((String::from(format!("[Text(\"{}\")]",(i)))))},
    )).id();        let regular_font_handle: Handle<Font> = Default::default();
commands.spawn((
    Text2d::new(format!("{}",(i))),
    TextColor(Color::WHITE),
    TextFont::from(regular_font_handle.clone()).with_font_size(24.),
    
  ChildOf(entit),Anchor::TOP_RIGHT,));
i += 1;
    }}
}