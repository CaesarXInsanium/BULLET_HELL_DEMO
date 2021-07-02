use amethyst::{
    ecs::{SystemDesc, System, SystemData, World, WriteStorage, Write, ReadStorage, Read, Join}
};

#[derive(amethyst::SystemDesc)]
struct MovePlayerSystem;

impl<'s> amethyst::prelude::System for MovePlayerSystem{
    type SystemData = ();

}