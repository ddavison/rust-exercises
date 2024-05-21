use inquire::{{validator::Validation, CustomType}, Select};

/// Room
struct Room {
    unit: &'static str,
    length: f32,
    width: f32
}

impl Room {
    const CONVERSION_FACTOR: f32 = 0.09290304;
    /// Area in feet²

    /// Instantiate a new room
    fn new(unit: &'static str, length: f32, width: f32) -> Self {
        Room { unit, length, width }
    }

    fn area_ft(&self) -> f32 {
        match self.unit {
            "feet" => self.length * self.width,
            "meters" => { (self.length * self.width) / Self::CONVERSION_FACTOR },
            _ => panic!("impossible?")
        }
    }

    /// Area in meters²
    fn area_me(&self) -> f32 {
        self.area_ft() * Self::CONVERSION_FACTOR
    }
}

/// Area of a Rectangular Room
///
/// Create a program that calculates the area of a room.
/// Prompt the user for the length and the width of the room in feet.
/// Then display the area in both square feet and square meters.
///
/// m² = f² * 0.09290304
/// f² = m² / 0.09290304
fn main() {
    let units: Vec<&str> = vec!["feet", "meters"];

    let unit = Select::new("Select the unit", units).prompt().unwrap();
    let length = CustomType::<f32>::new(format!("What is the length of the room in {unit}?").as_str())
        .with_error_message("Enter a valid number")
        .with_help_message(format!("Type the length (in {unit})").as_str())
        .with_validator(|val: &f32| {
            match *val {
                0.1.. => Ok(Validation::Valid),
                _ => Ok(Validation::Invalid("Invalid number".into()))
            }
        }).prompt().unwrap();

    let width = CustomType::<f32>::new(format!("What is the width of the room in {unit}?").as_str())
        .with_error_message("Enter a valid number")
        .with_help_message(format!("Type the width (in {unit})").as_str())
        .with_validator(|val: &f32| {
            match *val {
                0.1.. => Ok(Validation::Valid),
                _ => Ok(Validation::Invalid("Invalid number".into()))
            }
        }).prompt().unwrap();

    let room = Room::new(unit, length, width);
    println!("You entered dimensions of {} {} by {} {}.", &room.length, unit, &room.width, unit);
    println!("The area is: {} feet²; {} meters²", &room.area_ft(), &room.area_me());
}