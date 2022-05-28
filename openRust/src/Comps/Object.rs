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
pub trait Object<T> : ObjectClone<T>
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

    fn GetAttached(&self) -> T;
}

trait ObjectClone<T>
{
    fn cloneBox(&self) -> Box<dyn Object<T>>;    
}

impl<T,F> ObjectClone<F> for T
    where T: 'static + Object<F> + Clone,
{
    fn cloneBox(&self) -> Box<dyn Object<F>>
    {
        Box::new(self.clone())
    }
}

impl<T> Clone for Box<dyn Object<T>>
{
    fn clone(&self) -> Box<dyn Object<T>>
    {
        self.cloneBox()
    }
}

