use crate::Comps::TraitObj::*

struct ObjectBasic 
{
    pub name_ : &'static str
    pub prevState_ : TraitObj::ObjState,
    pub currState_ : TraitObj::ObjState,
    pub nextState_ : TraitObj::ObjState,

}
