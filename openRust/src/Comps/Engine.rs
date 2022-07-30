use crate::Comps::Object::*;
use crate::Comps::TransformComp::*;
use crate::Comps::WindowComp::*;

use std::collections::HashMap;

#[derive(Clone,Eq, Hash, PartialEq)]
pub enum CompType
{
    cTrans,
    cWin,
    
}

#[derive(Clone)]
pub struct Engine
{   
    pub components_ : HashMap<CompType, &'static Object>,  // Transform & Window.  

    // INTERFACE /////////// 
    //pub name_ : &'static str ,
    //pub prevState_ : ObjState,
    //pub currState_ : ObjState,
    //pub nextState_ : ObjState,

    ////////////////////////

    
}



// Inheritance through traits
impl Object for Engine
{
        // Return type is Self{}. 
    fn Create() -> Self // Ctor for self
    {
        return Self
        {
            name_ : "Engine",
            prevState_ : ObjState::oInvalid,
            currState_ : ObjState::oInvalid,
            nextState_ : ObjState::oStart, 
            components_ : HashMap::new(),           

        }
    }

    /*
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

    //fn GetAttached(&self) -> &Engine { self } */

}

impl Engine
{
    // Add components by name (or enum?)
    // Key for components map
    pub fn AddComp(&mut self, compType : CompType) 
    {        
        match compType
        {
            CompType::cTrans => 
            {
                let mut trans0 : TransformComp = TransformComp::Create();
                trans0.name_ = "Transform";
                self.components_
                    .insert(compType, &trans0);
            }
            CompType::cWin => 
            {
                let mut win0 : WindowComp = WindowComp::Create();
                win0.name_ = "Window";
                self.components_
                    .insert(compType, &win0);
            }
            _ => return
        }

    }

    // https://doc.rust-lang.org/std/option/enum.Option.html
    // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    // Hashmap get returns an option - 
    //  Some or None(nullptr). Rust has no null value.
    pub fn GetComp<compType>(&self, compKey : CompType) 
        -> &'static dyn Object //  
                                              
    {        
        let opt = self.components_.get(&compKey);
        let comp = opt.unwrap();
        return comp;

    }

}

/*
if self.components_.get(&compKey).is_some()
{
    self.components_.get(&compKey).unwrap()
}       
else
{
    None
}
*/