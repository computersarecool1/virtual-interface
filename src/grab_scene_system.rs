use bevy::{prelude::*, reflect::enums::Enum};
use serde::de::IntoDeserializer;
use crossbeam_channel::{bounded, Receiver};

use crate::{Code, EnumCode, hi, new_c::StreamReceiver};

pub struct grabPlugin;

impl Plugin for grabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load,load2).chain()).add_systems(Startup, (setup)).insert_resource(ee { scalex: vec![], scaley: vec![],x: vec![], y: vec![], code: vec![] }).insert_resource(Gb (false) );        

    }
}#[derive(Resource)]
struct ThreadSender(std::sync::mpsc::Sender<ee>);
fn load(
    mut n: ResMut<ee>,
    sender: Res<ThreadSender>,mut nn: ResMut<Gb>,
    entit: Query<(&Transform, &Code),Added<Code>>,mut commands: Commands
) {
    for (t,code) in entit {
        
        
        n.scalex.push(t.scale.x);
        n.scaley.push(t.scale.y);
        n.x.push(t.translation.x);
        n.y.push(t.translation.y);n.code.push(code.s.clone());
nn.0 = true;

    };
    
    
}
#[derive(Resource,Debug)]
pub struct Gb(bool);
fn load2( mut n: ResMut<ee>,mut nn: ResMut<Gb>,
    sender: Res<ThreadSender>,
    entit: Query<Entity, Added<Code>>,mut commands: Commands
) {    if nn.0 {
        
        sender.0.send(n.clone()).ok();println!("Thread ved: " );
    }
nn.0 = false;

    
}
  fn setup(mut commands: Commands) {
    let (tx, rx) = channel::<ee>();
    let (txtx, rxrx) = bounded::<Event>(1);

    commands.insert_resource(ThreadSender(tx));
commands.insert_resource(StreamReceiver(rxrx));
    std::thread::spawn(move || {            println!("Thread received: " );

        while let Ok(data) = rx.recv() {
         loadmain(txtx.clone(),data);
        }
    });
}
#[derive(Resource,Debug,Clone)]
pub struct ee {scalex: Vec<f32>,scaley: Vec<f32>,x: Vec<f32>,y: Vec<f32>,code: Vec<crate::EnumCode>}

use crate::{ G};
use rdev::{Button,Event, EventType,  grab};

use enigo::{
    
    Axis::{Horizontal, Vertical}, Coordinate::{Abs, Rel}, Direction::{Click, Press, Release}, Enigo, Key, Keyboard, Mouse, Settings, agent::Agent
};
use std::{
    sync::{Arc, Mutex, mpsc::{channel}},
    thread,
    time::Duration,
};

pub fn loadmain(mut sender: crossbeam_channel::Sender<Event>,mut nnnn: ee
) -> std::sync::mpsc::Receiver<bool> {    let (schan, rchan) = channel::<bool>();let mut enigo = Enigo::new(&Settings::default()).unwrap();let (mut x, mut y) = enigo.location().unwrap();
            let windowy: f32= 1079.;let m  = Arc::new(Mutex::new(enigo.location().unwrap()));let mm = Arc::clone(&m);
            let windowx: f32=1920.;                let locationx = Arc::new(Mutex::new(0.));let mut o = Arc::new(Mutex::new(false));let mmo = Arc::clone(&o);
    let locationy = Arc::new(Mutex::new(0.));let loc_x_thread = Arc::clone(&locationx);
    let loc_y_thread = Arc::clone(&locationy);
let _listener = thread::spawn(move || {    

    

    


    grab(move |event: Event| -> Option<Event> {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();let mut mmm = *mm.lock().unwrap();let mut mo = *mmo.lock().unwrap();
            let mut e: bool=false ;                       let mut lx = *loc_x_thread.lock().unwrap();
            let mut ly: f32 = *loc_y_thread.lock().unwrap();
            let (mut enx, mut eny ) = enigo.location().unwrap();let mut y=eny as f32;let mut x=enx as f32;x =x+lx;y =y +ly;    
 
    let mut ly_mut = loc_y_thread.lock().unwrap();
        let mut lx_mut = loc_x_thread.lock().unwrap();
let mut mmmm = mm.lock().unwrap();
let mut mmmo = mmo.lock().unwrap();


                             let swallow = matches!(
        event.event_type,
        EventType::ButtonRelease(Button::Right)
           
    );


    if swallow {                  
                  ( mmmm.0,mmmm.1)  = enigo.location().unwrap();                                                                
                                                       *mmmo=false

                      
}            

use std::sync::mpsc::{Receiver, channel};
    for t in 0..nnnn.scalex.len() {
let mut nd= (nnnn.scalex[t],nnnn.scaley[t],nnnn.x[t],nnnn.y[t],nnnn.code[t].clone());

             if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) >= nd.2 +(nd.0 * (-50.0))as f32 {

                if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) <= nd.2 as f32 {
                    if ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) >= nd.3 +(nd.1 *  (-50.0) ) as f32 && ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) <= nd.3  as f32||((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) <= nd.3 +(nd.1 *  (-50.0) ) as f32 && ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) >= nd.3  as f32{
                      
                         if *mmmo== false {sender.send(event.clone()); println!("nd");
thread::spawn( move || {let mut enigo = Enigo::new(&Settings::default()).unwrap();
let mut nnd = match nd.4.clone() {
    EnumCode::StringValue(string) => string,
    EnumCode::MyKey(string) => string,

};println!("{:?}",nnd);println!("{:?}",nd);
    let deserialized_tokens: Vec<_> = ron::from_str(&nnd).unwrap();
    for token in &deserialized_tokens {
        enigo.execute(token);//println!("{:?}",token);
    }

                    });};e=true;*mmmo=true;
                     
                    }else{};
                }else{};
            }else{};


}

                             let swallow = matches!(
        event.event_type,
        EventType::ButtonPress(Button::Right)
           
    );


    if swallow {                  
                  ( mmmm.0,mmmm.1)  = enigo.location().unwrap();                                                                
                                                       *mmmo=false

                      
}       
  for t in 0..nnnn.scalex.len() {
let mut nd= (nnnn.scalex[t],nnnn.scaley[t],nnnn.x[t],nnnn.y[t],nnnn.code[t].clone());

                  if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) >= nd.2 +(nd.0 * (-50.0))as f32 {

                if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) <= nd.2 as f32 {
                    if ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) >= nd.3 +(nd.1 *  (-50.0) ) as f32 && ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) <= nd.3  as f32||((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) <= nd.3 +(nd.1 *  (-50.0) ) as f32 && ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) >= nd.3  as f32{
if *mmmo== false { thread::spawn( move || {let mut enigo = Enigo::new(&Settings::default()).unwrap();  });
sender.send(event.clone());  println!("ndnd");}
                      e=true;*mmmo=true;}
                     else{};
                 
                }else{};
            }else{};


}

                   let swallow = matches!(
        event.event_type,
        EventType::ButtonPress(Button::Left)
           
    );

    if swallow {                  
                  ( mmmm.0,mmmm.1)  = enigo.location().unwrap();                                                                

                      
}       

let mut nd= (1.5,-1.5,-480.0,-500.5);

             if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) >= nd.2 +(nd.0 * (-50.0))as f32 {

                if (( mmmm.0 as f32/*lx  as f32*/)- windowx / 2.0) <= nd.2 as f32 {
                    if ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) <= nd.3 +(nd.1 *  (-50.0) ) as f32 {
                        if ((windowy - (mmmm.1 as f32/*ly as f32*/)) - windowy / 2.0) >= nd.3  as f32 {if swallow { thread::spawn( move || {let mut enigo = Enigo::new(&Settings::default()).unwrap();  });
 if swallow { println!("C");   return None; }       
}
                  }
                     else{};
                    }else{};
                }else{};
            }else{};



let (mut xx, mut yy) = enigo.location().unwrap();let mut swallow2 = matches!(
        event.event_type,
                                                                                     
                                                                             
        EventType::ButtonRelease(Button::Right));if *mmmo==false  { if          (( mmmm.0 as f32+lx  as f32)- windowx / 2.0) > -(windowx / 2.0) as f32 {
                if (( mmmm.0 as f32+lx  as f32)- windowx / 2.0) < (windowx / 2.0)as f32 {
                    if ((windowy - (mmmm.1 as f32+ly as f32)) - windowy / 2.0) < windowy / 2.0 as f32 {
                        if ((windowy - (mmmm.1 as f32+ly as f32)) - windowy / 2.0) > -(windowy / 2.0) as f32 {if swallow2 { *ly_mut  =*ly_mut as f32+(mmmm.1 as f32-yy as f32);    *lx_mut  =*lx_mut as f32+(mmmm.0 as f32-xx as f32);


        // send your boolean or event
        if let Err(err) = schan.send(e) {
            println!("Coulsend event {:?}", err);
        }


    } }
                    }
                }
            }}



    let swallow = matches!(
        event.event_type,
             EventType::ButtonPress(Button::Right)
            | EventType::ButtonRelease(Button::Right)
    );

   

    if swallow {
        
        // send your boolean or event
        if let Err(err) = schan.send(e) {
            // println!("Could not send event {:?}", err);
        }

        if e {
            // swallow only if e == true
            return None;
        }
    }
///
    // Forward everything else
    Some(event)
})
.expect("Could not grab left mouse events");                }    );

    let mut events: Vec<Event> = Vec::new();
   rchan

        

 
}