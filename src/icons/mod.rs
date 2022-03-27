#[macro_use] mod macros;

use macroquad::{prelude::Texture2D};

use crate::term::*;

icons!{
    paren_l => "parens/l.png",
    paren_r => "parens/r.png",

    unknown => "unknown.png",

    potion_i => "potions/i.png",
    potion_k => "potions/k.png",
    potion_s => "potions/s.png",
    potion_y => "potions/y.png",

    meat => "meat/meat.png",

    critter_bird_baby => "critters/bird/01.png",
    critter_bird_adult => "critters/bird/02.png",
    critter_bird_elder => "critters/bird/03.png",

    critter_dog_baby => "critters/dog/01.png",
    critter_dog_adult => "critters/dog/02.png",
    critter_dog_elder => "critters/dog/03.png",

    critter_fish_baby => "critters/fish/01.png",
    critter_fish_adult => "critters/fish/02.png",
    critter_fish_elder => "critters/fish/03.png",

    critter_reptile_baby => "critters/reptile/01.png",
    critter_reptile_adult => "critters/reptile/02.png",
    critter_reptile_elder => "critters/reptile/03.png",

    critter_robot_baby => "critters/robot/01.png",
    critter_robot_adult => "critters/robot/02.png",
    critter_robot_elder => "critters/robot/03.png",

    tftrigger_bird => "tftriggers/bird.png",
    tftrigger_dog => "tftriggers/dog.png",
    tftrigger_fish => "tftriggers/fish.png",
    tftrigger_reptile => "tftriggers/reptile.png",
    tftrigger_robot => "tftriggers/robot.png",
}

impl Icons {
    pub(crate) fn pick(&self, glyph: &Glyph) -> Texture2D {
        match glyph {
            Glyph::LParen => self.paren_l,
            Glyph::RParen => self.paren_r,
            Glyph::Unknown => self.unknown,

            Glyph::Single(atom) => {
                match atom {
                    Atom::Potion(p) => 
                        match p {
                            Potion::I => self.potion_i,
                            Potion::K => self.potion_k,
                            Potion::S => self.potion_s,
                            Potion::Y => self.potion_y,
                        }
                    Atom::Meat => self.meat,
                    Atom::Critter(caste, rank) => {
                        match caste {
                            Caste::Bird => match rank {
                                Rank::Baby => self.critter_bird_baby,
                                Rank::Adult => self.critter_bird_adult,
                                Rank::Elder => self.critter_bird_elder,
                            },
                            Caste::Dog => match rank {
                                Rank::Baby => self.critter_dog_baby,
                                Rank::Adult => self.critter_dog_adult,
                                Rank::Elder => self.critter_dog_elder
                            }
                            Caste::Fish => match rank {
                                Rank::Baby => self.critter_fish_baby,
                                Rank::Adult => self.critter_fish_adult,
                                Rank::Elder => self.critter_fish_elder
                            }
                            Caste::Reptile => match rank {
                                Rank::Baby => self.critter_reptile_baby,
                                Rank::Adult => self.critter_reptile_adult,
                                Rank::Elder => self.critter_reptile_elder
                            }
                            Caste::Robot => match rank {
                                Rank::Baby => self.critter_robot_baby,
                                Rank::Adult => self.critter_robot_adult,
                                Rank::Elder => self.critter_robot_elder
                            }
                        }
                    }
                    Atom::TFTrigger(caste) => match caste {
                        Caste::Bird => self.tftrigger_bird,
                        Caste::Dog => self.tftrigger_dog,
                        Caste::Fish => self.tftrigger_fish,
                        Caste::Reptile => self.tftrigger_reptile,
                        Caste::Robot => self.tftrigger_robot,
                    }
                }
            }
        }
    }
}