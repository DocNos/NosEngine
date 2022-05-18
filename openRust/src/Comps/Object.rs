
pub enum ObjState
{
    oInvalid,
    oStart,
    oUpdate,
    oStop,
    oRestart,
}


// T : object type (Transform, Engine, Window)
pub trait Object<T>
{
    fn Name(&self) -> str;
    fn CurrState(&self) -> ObjState;
    fn NextState(&self) -> ObjState;
    fn PrevState(&self) -> ObjState;

    fn Create(&self) -> &Object<T>;   
    fn CheckState(&self) -> ObjState;
    fn Update(&self, dt : u32);
    fn Destroy(&self);
}



