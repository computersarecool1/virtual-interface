use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use rand::{RngExt, SeedableRng};
pub struct New_c;

impl Plugin for New_c {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_visible)).add_message::<StreamMessage>()   .add_systems(FixedUpdate, read_stream);        

    }
}

#[derive(Resource, Deref)]
pub struct StreamReceiver(pub Receiver<Event>);
fn read_stream(receiver: Res<StreamReceiver>, mut events: MessageWriter<StreamMessage>) {
    for from_stream in receiver.try_iter() {
        events.write(StreamMessage(from_stream));
    }
}
#[derive(Message)]
struct StreamMessage(Event);


use std::{sync::mpsc::{self,Sender, channel}, thread};



use rdev::{Event, EventType, listen};

fn make_visible(mut reader: MessageReader<StreamMessage>, mut en: Query<(Entity)>,mouse: Res<ButtonInput<MouseButton>>,mut commands: Commands) {      for ( message) in reader.read(){
let mut nnd = match message.0.clone().event_type {
    EventType::ButtonPress(rdev::Button::Right) => {        for mut en in &mut en {

commands.entity(en).insert((Visibility::Visible)); 
        };

  } ,    EventType::ButtonRelease(rdev::Button::Right) => {

        for mut en in &mut en {

commands.entity(en).insert((Visibility::Hidden)); 
        };
  } , _ => {}

    

};
    }

}