#![allow(warnings, unused)]
extern crate nalgebra as na;
extern crate glium;

use na::{Vector2, Matrix};
use std::ptr;
mod Comps;
use crate::Comps::Engine::*;
use crate::Comps::Object::*;
use crate::Comps::TransformComp::*;

fn main() 
{
    let pos0 = Vector2 { x : 0, y : 0};
    

    // Engine<3> eng0;
    //let mut eng0 = Engine 
    //    { name_ : "" 
    //    , prevState_ : ObjState::oInvalid
    //    , currState_ : ObjState::oInvalid
    //    , nextState_ : ObjState::oInvalid };

    
    
    // Issue with the current virtualization found -
    // Storing comps as Objects in engine: 
    //  due to the heirarchy, cannot convert to TransformComp.
    // 
    let mut eng0 = Engine::Create();
    eng0.AddComp(CompType::cTrans);
    // let trans0 : &TransformComp = 
    //    eng0.GetComp("Transform") as &TransformComp;
    let trans0 : &TransformComp = eng0.GetTrans();
        
    println!("{}" , eng0.name_);
    // println!("{}" , trans0.name_);
}
