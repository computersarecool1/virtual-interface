use std::f32::consts::PI;

use bevy::{audio::Volume, prelude::*, sprite::Anchor};
use enigo::{Enigo, Key, Settings, agent::{Agent, Token}};

use crate::{Code, EnumCode, NeedsTextChange, hi};

pub struct MyPlugin;
fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
  volume.volume = Volume::Linear(0.005);
}
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, change_global_volume).add_systems(Update, (load_scene_system,load,(boxAsset,system).chain(),filp)).insert_resource(GlobalVolume::new(Volume::Linear(0.005))).init_resource::<min5s::Mytime>();        

    }
}
fn system(entit: Query<(Entity,&Code,Has<NeedsTextChange>), (With<NeedsSpriteChange>)>,mut commands: Commands, asset_server: Res<AssetServer>) {

for (a,entit) in entit.iter().enumerate() {let mut enigo = Enigo::new(&Settings::default()).unwrap();
let mut nnd = match entit.1.s.clone() {
    EnumCode::StringValue(string) => string,
    EnumCode::MyKey(string) => string,

};
    let deserialized_tokens = ron::from_str(&nnd);
    let deserialized_tokens: Vec<_> = match deserialized_tokens {
        Ok(t) => t,
        Err(t) => {continue},
    };
    for token in &deserialized_tokens {


       let a = match token {
  
    Token::Text( string) => string,
    Token::Key( Key::Unicode(string),Click ) => {&string.to_string()}

    _ => { commands.entity(entit.0).insert((Sprite::from_image(asset_server.load(format!("e/{}ractal.png",a))))); continue;},
        };if entit.2 {
        let regular_font_handle: Handle<Font> = Default::default();
commands.spawn((        Transform::from_translation(Vec3 { x: 0., y: 0., z: 32. }),

    Text2d::new(format!("{}",a)),
    TextColor(Color::WHITE), 
    TextFont::from(regular_font_handle.clone()).with_font_size(40.),
    NeedsTextChange,
  ChildOf(entit.0),Anchor::BOTTOM_RIGHT,));
 

}

  
    }
    };


}

fn boxAsset (mut commands: Commands,mut entit: Query<(Entity),With<NeedsSpriteChange>>,asset_server: Res<AssetServer>) {
for mut entit in  entit {
     commands.spawn((Sprite::from_image(asset_server.load(format!("rrr.png"))),
    
  ChildOf(entit),Anchor::TOP_RIGHT,));commands.entity(entit).remove::<NeedsSpriteChange>();
}
}

fn filp (time: Res<Time>,mut rr: Query<(&mut Transform),(Added<Text2d>,With<NeedsTextChange>)>) {

    for mut i in rr {
                   i.rotate_x(PI); 

    }
} 
fn load(entit: Query<Entity, Added<Code>>,mut commands: Commands, asset_server: Res<AssetServer>) {
                 for (entit) in entit {commands.entity(entit).insert((NeedsSpriteChange));println!("no loo");};
}
const SCENE_FILE_PATH: &str = "triggerpathtriggerpathtriggerpath";
#[derive(Component)]
struct NeedsSpriteChange;
fn load_scene_system(
mut reader: MessageReader<hi>,mut commands: Commands, asset_server: Res<AssetServer> ,    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x = 1.;
    let y = 1.;
    let width = -22.;
    let height = -22.;
  
    for hi(e) in reader.read() {  commands.spawn((DynamicWorldRoot(asset_server.load(SCENE_FILE_PATH))),


    );}


}


pub mod min5s{
    use std::time::Duration;

use bevy::{input::mouse::MouseButtonInput, prelude::*};


#[derive(Resource, Default)]
pub struct Mytime (f32);
pub fn min5(time: ResMut<Time>,mut mytime: ResMut<Mytime>,mut pitch_assets: ResMut<Assets<Pitch>>,mut commands: Commands) {

    if time.elapsed_secs() >= mytime.0 as f32 {
            commands.spawn((
        AudioPlayer(pitch_assets.add(Pitch::new(520.0, Duration::new(1, 0)))),
        PlaybackSettings::DESPAWN,
    ));
    }
}

pub fn min5reset(mut time: ResMut<Time>,mut input: MessageReader<MouseButtonInput>,mut mytime: ResMut<Mytime>) {

   for input in input.read()  {
       mytime.0 = time.elapsed_secs() + (60*22) as f32;
  }
}

}