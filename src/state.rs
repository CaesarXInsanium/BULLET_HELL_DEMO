use amethyst::{self, GameData, SimpleState, SimpleTrans, StateData, StateEvent};
mod init_functions;



#[derive(Default)]
pub struct GameplayState{
    score: u32
}

impl SimpleState for GameplayState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        init_functions::init_camera(world);

    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>){

    }
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans{
        todo!()
    }
    fn fixed_update(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>
    ) -> SimpleTrans{
        todo!()
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        todo!()
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn shadow_fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn shadow_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}