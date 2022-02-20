pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            return Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut mana) => {
                if *mana < mana_cost {
                    return 0;
                } else {
                    *mana -= mana_cost;
                    return 2 * mana_cost;
                }
            },
            None => {
                if self.health <= mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                return 0;
            }
        }
    }
}
