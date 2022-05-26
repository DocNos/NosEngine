// Need partialEq for compare : == , =!
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ObjState
{
    oInvalid,
    oStart,
    oUpdate,
    oStop,
    oRestart,
}


// T : object type (Transform, Engine, Window)

pub trait Object
{
    fn Name(&self) -> &str;
    fn CurrState(&self) -> ObjState;
    fn NextState(&self) -> ObjState;
    fn PrevState(&self) -> ObjState;

    fn Create() -> Self;   
    fn CheckState(&mut self) -> ObjState;
    fn Update(&mut self, dt : u32);
    fn Destroy(&mut self);
}



