use crate::Comps::Object::*;
use ObjState;


pub struct Engine<'a>
{   
    // INTERFACE /////////// 
    name_ : &'a str,
    prevState_ : ObjState,
    currState_ : ObjState,
    nextState_ : ObjState,

    ////////////////////////

    //components_ : [Object<T>; 2],  // Transform & Window.  
}



// Inheritance through traits
impl Object for Engine<'a>
{
    fn Name(&self)      -> &'a str { self.name_ } // "Unbox" value aka dereference
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }



    fn Create(&self) -> &dyn Object
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

    fn Update(&self, dt : u32)
    {
        self.CheckState(self);
    }

}

