use crate::Comps::Object::*;

pub struct Engine
{   
    // INTERFACE /////////// 
    name_ : str,
    prevState_ : ObjState,
    currState_ : ObjState,
    nextState_ : ObjState,

    ////////////////////////

    components_ : [Object<T>; 2],  // Transform & Window.  
}

impl Engine
{

}

// Inheritance through traits
impl Object for Engine
{
    fn Name(&self)      -> str { self.name_ }
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }



    fn Create<Engine : i8 >(&self) 
        -> &Object<Engine<i8>>
    {
        self.name_ = "Engine";
        self.nextState_ = oUpdate;
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

