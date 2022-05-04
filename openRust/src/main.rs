#![allow(warnings, unused)]
extern crate nalgebra as na;
use na::{Vector2, Matrix};
use std::ptr;
mod TransformNE;
use TransformNE::transNos;

fn main() 
{
    let pos0 = Vector2 { x : 0, y : 0};
    let trans0 = 
        transNos{ pos_      : Vector2 {x:5.0, y:-5.0} 
                , rotation_ : Vector2{x:0.0, y:0.0}
                , matrix_   : ptr::null() 
                };
    println!("Hello, world!");
}
