#[derive(Debug, Hash, PartialEq, Eq)]
struct Location {
    x: i16,
    y: i16,
}

#[derive(Debug)]
enum LocationData {
    Asteroid,
    Empty,
}

impl From<char> for LocationData {
    fn from(c: char) -> Self {
        match c {
            '#' => LocationData::Asteroid,
            '.' => LocationData::Empty,
            x => panic!("Unknown symbol {}", x)
        }
    }
}

#[derive(Default, Debug)]
struct Field {
    locations: std::collections::HashMap::<Location, LocationData>,
}

impl Field {
    fn new_from_content(content: String) -> Self {
        let mut field = Self::default();
        field.locations = std::collections::HashMap::<_, _>::new();
        let mut y : i16 = 0;
        for line in content.lines() {
            let mut x : i16 = 0;
            for c in line.chars() {
                let location = Location { x, y };
                field.locations.insert(location, c.into());
                x += 1;
            }
            y += 1;
        }
        field
    }
}

fn main() {
    let content = std::fs::read_to_string("inputs.txt").unwrap();

    let mut field = Field::new_from_content(content);
    println!("{:?}", field);
}
