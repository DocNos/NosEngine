#![allow(warnings, unused)]
extern crate nalgebra as na;
extern crate glium;

use na::{Vector2, Matrix};
use std::ptr;
mod Comps;
use crate::Comps::Engine::*;
use crate::Comps::Object::*;
//use Object::Objstate;
fn main() 
{
    let pos0 = Vector2 { x : 0, y : 0};
    

    // Engine<3> eng0;
    let mut eng0 = Engine 
        { name_ : "" 
        , prevState_ : ObjState::oInvalid
        , currState_ : ObjState::oInvalid
        , nextState_ : ObjState::oInvalid };

    //let mut eng1 : Engine = eng1.Create();
    
    eng0.Create();
    println!("{}" , eng0.name_);
}
