#[derive(Debug)]
pub enum ScrollSpeed {
    Speed1,
    Speed2,
}

impl ScrollSpeed {
    // Helper function to increment the speed
    pub fn increment(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed1 => ScrollSpeed::Speed2,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed2,
        };
    }

    // Helper function to decrement the speed
    pub fn decrement(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed1 => ScrollSpeed::Speed1,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed1,
        };
    }

    pub fn get_zoom_scale(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed1 => 1.15,
            ScrollSpeed::Speed2 => 1.0,
        }
    }

    pub fn get_scroll_speed(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed1 => 5.0,
            ScrollSpeed::Speed2 => 5.0,
        }
    }
}

#[derive(Debug)]
pub enum EnemyType {
    Type0,
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
}

impl EnemyType {
    pub fn get_image_file(&self) -> String {
        match self {
            EnemyType::Type0 => "ships/ship_0000.png".to_string(),
            EnemyType::Type1 => "ships/ship_0001.png".to_string(),
            EnemyType::Type2 => "ships/ship_0002.png".to_string(),
            EnemyType::Type3 => "ships/ship_0003.png".to_string(),
            EnemyType::Type4 => "ships/ship_0004.png".to_string(),
            EnemyType::Type5 => "ships/ship_0005.png".to_string(),
            EnemyType::Type6 => "ships/ship_0006.png".to_string(),
            EnemyType::Type7 => "ships/ship_0007.png".to_string(),
            EnemyType::Type8 => "ships/ship_0008.png".to_string(),
            EnemyType::Type9 => "ships/ship_0009.png".to_string(),
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            EnemyType::Type0 => 1.0,
            EnemyType::Type1 => 2.1,
            EnemyType::Type2 => 3.4,
            EnemyType::Type3 => 4.2,
            EnemyType::Type4 => 5.5,
            EnemyType::Type5 => 6.7,
            EnemyType::Type6 => 7.0,
            EnemyType::Type7 => 8.1,
            EnemyType::Type8 => 9.0,
            EnemyType::Type9 => 13.5,
        }
    }
}
