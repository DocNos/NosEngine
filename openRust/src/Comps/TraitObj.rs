

pub trait ObjBasic
{
    fn name(&self) -> &str;
    fn init(&self, dt: f32);
    fn update(&self, dt : f32);
    fn exit(&self, dt : f32);
}

pub trait ObjStates
{
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

    fn getState(&self) -> ObjState;
    fn setState(&self, ObjState);
}


