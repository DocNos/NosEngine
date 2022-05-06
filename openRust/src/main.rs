#![allow(warnings, unused)]
extern crate nalgebra as na;
use na::{Vector2, Matrix};
use std::ptr;
mod TransformNE;
use TransformNE::transNos;
mod Window;

fn main() 
{
    let win0 : Window();
    println!("Hello, world!");
}
