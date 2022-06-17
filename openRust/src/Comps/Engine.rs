use crate::Comps::Object::*;
use ObjState;
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
    // INTERFACE /////////// 
    pub name_ : &'static str ,
    pub prevState_ : ObjState,
    pub currState_ : ObjState,
    pub nextState_ : ObjState,

    ////////////////////////

    pub components_ : HashMap<CompType, Box<dyn Object>>,  // Transform & Window.  
}



// Inheritance through traits
impl Object for Engine
{
    fn Name(&self)      -> &'static str { self.name_ } // "Unbox" value aka dereference
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }


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
                    .insert(compType, Box::new(trans0));
            }
            CompType::cWin => 
            {
                let mut win0 : WindowComp = WindowComp::Create();
                win0.name_ = "Window";
                self.components_
                    .insert(compType, Box::new(win0));
            }
            _ => return
        }

    }

    // https://doc.rust-lang.org/std/option/enum.Option.html
    // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    // Hashmap get returns an option - 
    //  Some or None(nullptr). Rust has no null value.
    //pub fn GetComp(&self, compKey : &'static str) 
    //    -> &'static Box<dyn Object + '_>
    //{        
    //    self.components_.get(&compKey).unwrap()         
    //}

    pub fn GetTrans(&self) -> &TransformComp
    {       
        let comp = self.components_.get(&CompType::cTrans)
            .unwrap();
        return Box::into_raw(comp);

    }

}

