use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AchievementType {
    Tile128,
    Tile256,
    Tile512,
    Tile1024,
    Tile2048,
    Score1000,
    Score3000,
    Score5000,
    Score7000,
    Score10000,
    Score15000,
    Score20000,
}

impl AchievementType {
    pub fn get_name(&self) -> &'static str {
        match self {
            AchievementType::Tile128 => "合成128",
            AchievementType::Tile256 => "合成256",
            AchievementType::Tile512 => "合成512",
            AchievementType::Tile1024 => "合成1024",
            AchievementType::Tile2048 => "合成2048",
            AchievementType::Score1000 => "达到1000分",
            AchievementType::Score3000 => "达到3000分",
            AchievementType::Score5000 => "达到5000分",
            AchievementType::Score7000 => "达到7000分",
            AchievementType::Score10000 => "达到10000分",
            AchievementType::Score15000 => "达到15000分",
            AchievementType::Score20000 => "达到20000分",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            AchievementType::Tile128 => "首次合成出128方块",
            AchievementType::Tile256 => "首次合成出256方块",
            AchievementType::Tile512 => "首次合成出512方块",
            AchievementType::Tile1024 => "首次合成出1024方块",
            AchievementType::Tile2048 => "首次合成出2048方块",
            AchievementType::Score1000 => "累计达到1000分",
            AchievementType::Score3000 => "累计达到3000分",
            AchievementType::Score5000 => "累计达到5000分",
            AchievementType::Score7000 => "累计达到7000分",
            AchievementType::Score10000 => "累计达到10000分",
            AchievementType::Score15000 => "累计达到15000分",
            AchievementType::Score20000 => "累计达到20000分",
        }
    }

    pub fn is_score(&self) -> bool {
        match self {
            AchievementType::Score1000 | AchievementType::Score3000 |
            AchievementType::Score5000 | AchievementType::Score7000 |
            AchievementType::Score10000 | AchievementType::Score15000 |
            AchievementType::Score20000 => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            AchievementType::Tile128 => 128,
            AchievementType::Tile256 => 256,
            AchievementType::Tile512 => 512,
            AchievementType::Tile1024 => 1024,
            AchievementType::Tile2048 => 2048,
            AchievementType::Score1000 => 1000,
            AchievementType::Score3000 => 3000,
            AchievementType::Score5000 => 5000,
            AchievementType::Score7000 => 7000,
            AchievementType::Score10000 => 10000,
            AchievementType::Score15000 => 15000,
            AchievementType::Score20000 => 20000,
        }
    }
}

pub struct Achievements {
    unlocked: HashSet<AchievementType>,
    new_achievements: Vec<AchievementType>,
}

impl Achievements {
    pub fn new() -> Achievements {
        Achievements {
            unlocked: Self::load_unlocked(),
            new_achievements: Vec::new(),
        }
    }

    fn load_unlocked() -> HashSet<AchievementType> {
        let path = Path::new("achievements.txt");
        if !path.exists() {
            return HashSet::new();
        }

        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return HashSet::new(),
        };

        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            return HashSet::new();
        }

        let mut unlocked = HashSet::new();
        for line in content.lines() {
            let achievement = match line.parse::<i32>() {
                Ok(0) => AchievementType::Tile128,
                Ok(1) => AchievementType::Tile256,
                Ok(2) => AchievementType::Tile512,
                Ok(3) => AchievementType::Tile1024,
                Ok(4) => AchievementType::Tile2048,
                Ok(5) => AchievementType::Score1000,
                Ok(6) => AchievementType::Score3000,
                Ok(7) => AchievementType::Score5000,
                Ok(8) => AchievementType::Score7000,
                Ok(9) => AchievementType::Score10000,
                Ok(10) => AchievementType::Score15000,
                Ok(11) => AchievementType::Score20000,
                _ => continue,
            };
            unlocked.insert(achievement);
        }

        unlocked
    }

    fn save_unlocked(&self) {
        let path = Path::new("achievements.txt");
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(_) => return,
        };

        for achievement in &self.unlocked {
            let id = match achievement {
                AchievementType::Tile128 => 0,
                AchievementType::Tile256 => 1,
                AchievementType::Tile512 => 2,
                AchievementType::Tile1024 => 3,
                AchievementType::Tile2048 => 4,
                AchievementType::Score1000 => 5,
                AchievementType::Score3000 => 6,
                AchievementType::Score5000 => 7,
                AchievementType::Score7000 => 8,
                AchievementType::Score10000 => 9,
                AchievementType::Score15000 => 10,
                AchievementType::Score20000 => 11,
            };
            file.write_all(format!("{}\n", id).as_bytes()).ok();
        }
    }

    pub fn check_tile_achievement(&mut self, tile_value: i32) {
        let achievement = match tile_value {
            128 => Some(AchievementType::Tile128),
            256 => Some(AchievementType::Tile256),
            512 => Some(AchievementType::Tile512),
            1024 => Some(AchievementType::Tile1024),
            2048 => Some(AchievementType::Tile2048),
            _ => None,
        };

        if let Some(ach) = achievement {
            if !self.unlocked.contains(&ach) {
                self.unlocked.insert(ach);
                self.new_achievements.push(ach);
                self.save_unlocked();
            }
        }
    }

    pub fn check_score_achievement(&mut self, score: i32) {
        let achievements = [
            AchievementType::Score1000,
            AchievementType::Score3000,
            AchievementType::Score5000,
            AchievementType::Score7000,
            AchievementType::Score10000,
            AchievementType::Score15000,
            AchievementType::Score20000,
        ];

        for &achievement in &achievements {
            if score >= achievement.get_value() && !self.unlocked.contains(&achievement) {
                self.unlocked.insert(achievement);
                self.new_achievements.push(achievement);
                self.save_unlocked();
            }
        }
    }

    pub fn has_new_achievements(&self) -> bool {
        !self.new_achievements.is_empty()
    }

    pub fn take_new_achievements(&mut self) -> Vec<AchievementType> {
        std::mem::take(&mut self.new_achievements)
    }

    pub fn is_unlocked(&self, achievement: AchievementType) -> bool {
        self.unlocked.contains(&achievement)
    }

    pub fn get_all_achievements() -> Vec<AchievementType> {
        vec![
            AchievementType::Tile128,
            AchievementType::Tile256,
            AchievementType::Tile512,
            AchievementType::Tile1024,
            AchievementType::Tile2048,
            AchievementType::Score1000,
            AchievementType::Score3000,
            AchievementType::Score5000,
            AchievementType::Score7000,
            AchievementType::Score10000,
            AchievementType::Score15000,
            AchievementType::Score20000,
        ]
    }

    pub fn get_unlocked_count(&self) -> usize {
        self.unlocked.len()
    }

    pub fn get_total_count() -> usize {
        12
    }
}