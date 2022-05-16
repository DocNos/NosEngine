#![allow(warnings, unused)]
extern crate nalgebra as na;
extern crate glium;

use na::{Vector2, Matrix};
use std::ptr;
mod Engine;

fn main() 
{
    let pos0 = Vector2 { x : 0, y : 0};
    // let trans0 = 
    //    TransComp{ pos_      : Vector2 {x:5.0, y:-5.0} 
    //            , rotation_ : Vector2{x:0.0, y:0.0}
    //            };
    //let win0 = Window::CreateWindow();

    let eng0 : Engine<3>;
    println!("Hello, world!");
}
