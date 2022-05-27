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
pub trait Object : ObjectClone
{
    fn Name(&self) -> &str;
    fn CurrState(&self) -> ObjState;
    fn NextState(&self) -> ObjState;
    fn PrevState(&self) -> ObjState;
                // where -> specifying constraints on lifetime & generic (template) parameters
    fn Create() -> Self where Self: Sized;   
    fn CheckState(&mut self) -> ObjState;
    fn Update(&mut self, dt : u32);
    fn Destroy(&mut self);
}

trait ObjectClone
{
    fn cloneBox(&self) -> Box<dyn Object>;    
}

impl<T> ObjectClone for T
    where T: 'static + Object + Clone,
{
    fn cloneBox(&self) -> Box<dyn Object>
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object>
{
    fn clone(&self) -> Box<dyn Object>
    {
        self.cloneBox()
    }
}

