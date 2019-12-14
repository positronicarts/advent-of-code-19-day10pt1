#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Location {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum LocationData {
    Asteroid,
    Empty,
}

impl From<char> for LocationData {
    fn from(c: char) -> Self {
        match c {
            '#' => LocationData::Asteroid,
            '.' => LocationData::Empty,
            x => panic!("Unknown symbol {}", x),
        }
    }
}

#[derive(Default, Debug)]
struct Field {
    locations: std::collections::HashMap<Location, LocationData>,
    asteroids: Vec<Location>,
}

impl Field {
    fn new_from_content(content: String) -> Self {
        let mut field = Self::default();
        field.locations = std::collections::HashMap::<_, _>::new();
        field.asteroids = Vec::<_>::new();
        for (y, line) in content.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let location = Location { x: x as i16, y: y as i16};
                let data: LocationData = c.into();
                field.locations.insert(location.clone(), data.clone());
                if data == LocationData::Asteroid {
                    field.asteroids.push(location);
                }
            }
        }
        field
    }

    fn can_see(&self, from: &Location, to: &Location) -> bool {
        if self.locations[&to] != LocationData::Asteroid {
            return false;
        };
        if self.locations[&from] != LocationData::Asteroid {
            return false;
        };
        if to == from {
            return false;
        }
        let dx = from.x - to.x;
        let dy = from.y - to.y;

        for possible_clash in self.asteroids.iter() {
            if possible_clash == to {
                continue;
            }
            if possible_clash == from {
                continue;
            }
            let cdx = from.x - possible_clash.x;
            let cdy = from.y - possible_clash.y;

            // Closer?
            if (cdx * cdx + cdy * cdy) < (dx * dx + dy * dy) {
                // Same angle?
                if (cdx * dy == cdy * dx)
                    && (((dx >= 0) && (cdx >= 0)) || ((dx <= 0) && (cdx <= 0)))
                    && (((dy >= 0) && (cdy >= 0)) || ((dy <= 0) && (cdy <= 0)))
                {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let content = std::fs::read_to_string("inputs.txt").unwrap();

    let field = Field::new_from_content(content);
    let max = field
        .locations
        .keys()
        .map(|from| {
            field
                .locations
                .keys()
                .filter(|to| field.can_see(from, to))
                .count()
        })
        .max();

    println!("Max is {:?}", max.unwrap());
}
