use nalgebra::{Vector2, Matrix};
use crate::Comps::Object::*;

#[derive(Copy, Clone)]
pub struct TransformComp
{    
    // INTERFACE /////////// 
    //pub name_ : &'static str ,
    //pub prevState_ : ObjState,
    //pub currState_ : ObjState,
    //pub nextState_ : ObjState,

    ////////////////////////

    pub pos_ : Vector2<f32>,
    pub rotation_: Vector2<f32>,
    // dyn keyword - Matrix is a trait object
    //      dyn is object safe
    // U3 is a unsigned int 3 - 
    //      alias for Const<3>
    //pub matrix_: dyn DMatrix<f32, U3, U3>,
}

impl Object for TransformComp
{
    fn Create() -> Self
    {
        return Self
        {
            //name_ : "Transform",
            //prevState_ : ObjState::oInvalid,
            //currState_ : ObjState::oInvalid,
            //nextState_ : ObjState::oStart,  
            pos_ : Vector2 { x : 0.0, y : 0.0},
            rotation_ : Vector2 { x : 0.0, y : 0.0},          

        }
    }

    //fn GetAttached(&self) -> &TransformComp { self }

}

impl TransformComp
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