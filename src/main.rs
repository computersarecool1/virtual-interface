mod grab_scene_system; // import hello.rs
mod load_scene_system;
mod load_scene;

mod manmann;
mod new_c;
mod manmann_copy; use grab_scene_system::grabPlugin;
use load_scene_system::MyPlugin;use rdev::{Button,Event, EventType,  grab};
use bevy::{math::CompassOctant, prelude::*,     window::{
        CursorGrabMode, CursorIcon, CursorOptions, PresentMode, SystemCursorIcon, WindowLevel,
        WindowTheme,
    },};use enigo::{
    
    Axis::{Horizontal, Vertical}, Coordinate::{Abs, Rel}, Direction::{Click, Press, Release}, Enigo, Key, Keyboard, Mouse, Settings
};
use rdev::display_size;use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Player {
    name: String,
    health: u32,
}#[derive(Component, Debug, Clone,Reflect,serde::Deserialize)]
#[reflect(Component)]

enum EnumCode {
    StringValue(String),MyKey(String)

}
fn load_player() -> Player {
    let text = fs::read_to_string("player.json").unwrap();
    let player: Player = serde_json::from_str(&text).unwrap();
    player
}#[derive(Component, Debug, Clone,Reflect,serde::Deserialize)]
#[reflect(Component,Deserialize)]
pub struct NeedsTextChange;
#[derive(Component, Debug, Clone,Reflect,serde::Deserialize)]
#[reflect(Component,Deserialize)]

pub struct Code {
    s: EnumCode,

}
fn main(   
) {       

        
            unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
    }
       App::new().insert_resource(G(true)).add_message::<hi>().add_plugins((loadPlugin)).add_plugins((New_c)).add_plugins(MyPlugin).add_plugins(grabPlugin).add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_cursor_options: Some(CursorOptions{ grab_mode: CursorGrabMode::None, visible:true, hit_test: false  }),
            primary_window: Some(Window {
                position: WindowPosition::At(ivec2(0, 0)),
                transparent: true,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                decorations: false,resolution: (1920 as u32, 1079 as u32).into(),
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                #[cfg(target_os = "linux")]
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                ..default()
            }),
            ..default()
        })).insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, (a))
        .add_systems(Update, (d))     
     
        .run();
        
    }

#[derive(Message)]

pub struct hi(bool);
use rdev::listen;
use std::{
    sync::{Arc, Mutex, mpsc::channel},
    thread,
    time::Duration,
};

#[derive(Resource,Debug)]
pub struct G(bool);
fn a(mut writerhi: MessageWriter<hi>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
manmann::fractal(   
);
  //  commands.spawn((
   //     Sprite::from_image(asset_server.load("werft s.png")),
  //      Anchor::CENTER,
  //      Name::new("ww"),
    //    Transform::from_scale(Vec3 { x:0.24590163934426229508196721311475 as f32, y:0.24590163934426229508196721311475 as f32, z: (-22.) }).with_translation(Vec3 { x:0 as f32, y:0 as f32, z: (0.1) }),

        

//    ));
 
writerhi.write(hi(true));println!("99999evedevedevedLefteredfterftered eved ev event");
}fn d(mut event: ResMut<G>) {
    if event.0 {
        println!("events {:?}", event.0);
        // clear the flag so we don't print it again next frame
        event.0 = false;
    }
}
     use bevy::sprite::Anchor;

    use crate::{grab_scene_system::loadmain, load_scene::loadPlugin, new_c::New_c};



