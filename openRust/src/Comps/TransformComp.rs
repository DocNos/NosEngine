use nalgebra::{Vector2, Matrix};
use crate::Comps::Object::*;

#[derive(Copy, Clone)]
pub struct TransComp<'a>
{    
    // INTERFACE /////////// 
    pub name_ : &'a str ,
    pub prevState_ : ObjState,
    pub currState_ : ObjState,
    pub nextState_ : ObjState,

    ////////////////////////

    pub pos_ : Vector2<f32>,
    pub rotation_: Vector2<f32>,
    // dyn keyword - Matrix is a trait object
    //      dyn is object safe
    // U3 is a unsigned int 3 - 
    //      alias for Const<3>
    //pub matrix_: dyn DMatrix<f32, U3, U3>,
}

impl<'a> Object for TransComp<'a>
{
    fn Name(&self)      -> &'a str { self.name_ }
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }

    fn Create(&mut self) -> &dyn Object 
    {
        self.name_ = "Transform";
        self.nextState_ = ObjState::oUpdate;
        return self;
    }
    fn CheckState(&mut self) -> ObjState { self.currState_ }
    fn Update(&mut self, dt: u32) {}
    fn Destroy(&mut self) {}


}

impl<'a> TransComp<'a>
{
    fn SetPos(&mut self, _pos : Vector2<f32>)
    {
        self.pos_ = _pos;
    }
    fn AddPos(&mut self, _posAdd: Vector2<f32>)
    {
        self.pos_ += _posAdd;
    }
    fn GetPos(&self) -> Vector2<f32>
    {
        // to return implicitly, no semicolon.
        self.pos_
    }

}