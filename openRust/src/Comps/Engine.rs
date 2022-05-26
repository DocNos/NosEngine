use crate::Comps::Object::*;
use ObjState;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Engine<'a>
{   
    // INTERFACE /////////// 
    pub name_ : &'a str ,
    pub prevState_ : ObjState,
    pub currState_ : ObjState,
    pub nextState_ : ObjState,

    ////////////////////////

    components_ : &'a HashMap<&'a str, dyn Object>,  // Transform & Window.  
}



// Inheritance through traits
impl<'a> Object for Engine<'a>
{
    fn Name(&self)      -> &'a str { self.name_ } // "Unbox" value aka dereference
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }



    fn Create(&mut self) -> &dyn Object
    {
        self.name_ = "Engine";
        self.nextState_ = ObjState::oUpdate;
        return self;
    }

    fn CheckState(&mut self) -> ObjState
    {
        if self.nextState_ != self.currState_
        {
            self.prevState_ = self.currState_;
            self.currState_ = self.nextState_;
        }
        self.currState_
    }

    fn Update(&mut self, dt : u32)
    {
        // Self is implied on trait (member) fn call
        self.CheckState();
    }

    fn Destroy(&mut self) {}

}

impl<'a> Engine<'a>
{
    fn AddComp(objType : &'a str) 
    {

    }
}

