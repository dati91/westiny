use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

use bitflags;
use crate::metric_dimension::length::Meter;
use amethyst::core::num::Zero;

const SELECTIONS: [InputFlags; 4] = [
    InputFlags::SELECT1,
    InputFlags::SELECT2,
    InputFlags::SELECT3,
    InputFlags::SELECT4,
];

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct InputFlags: u16 {
        const NOP =         0b0000_0000_0000_0000;
        const FORWARD =     0b0000_0000_0000_0001;
        const BACKWARD =    0b0000_0000_0000_0010;
        const STRAFELEFT =  0b0000_0000_0000_0100;
        const STRAFERIGHT = 0b0000_0000_0000_1000;
        const SHOOT =       0b0000_0000_0001_0000;
        const USE =         0b0000_0000_0010_0000;
        const RUN =         0b0000_0000_0100_0000;
        const RELOAD =      0b0000_0000_1000_0000;
        const SELECT1 =     0b0000_0001_0000_0000;
        const SELECT2 =     0b0000_0010_0000_0000;
        const SELECT3 =     0b0000_0100_0000_0000;
        const SELECT4 =     0b0000_1000_0000_0000;
        const UP =          0b0001_0000_0000_0000;
        const DOWN =        0b0010_0000_0000_0000;
        const LEFT =        0b0100_0000_0000_0000;
        const RIGHT =       0b1000_0000_0000_0000;
    }

}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Input
{
    pub flags : InputFlags,
    pub cursor : Point2<Meter>
}

impl Input {
    /// returns the first SELECT value if any of them is active. Otherwise returns None
    pub fn get_selection(&self) -> Option<&InputFlags> {
        SELECTIONS.iter().find(|&select| self.flags.intersects(*select))
    }
}

impl Default for Input
{
    fn default() -> Self {
        Input{
            flags: InputFlags::NOP,
            cursor: Point2::new(Meter::zero(), Meter::zero()),
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
