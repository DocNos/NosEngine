
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
    fn Name(&self) -> str;
    fn CurrState(&self) -> ObjState;
    fn NextState(&self) -> ObjState;
    fn PrevState(&self) -> ObjState;

    fn Create(&self) -> &dyn Object;   
    fn CheckState(&self) -> ObjState;
    fn Update(&self, dt : u32);
    fn Destroy(&self);
}



