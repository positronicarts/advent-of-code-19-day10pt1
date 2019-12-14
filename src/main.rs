#[derive(Debug)]
enum Location {
    Asteroid,
    Empty,
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '#' => Location::Asteroid,
            '.' => Location::Empty,
            x => panic!("Unknown symbol {}", x)
        }
    }
}

#[derive(Default, Debug)]
struct Field {
    locations: std::collections::HashMap::<(i16, i16), Location>,
}

impl Field {
    fn new_from_content(content: String) -> Self {
        let mut field = Self::default();
        field.locations = std::collections::HashMap::<_, _>::new();
        let mut y : i16 = 0;
        for line in content.lines() {
            let mut x : i16 = 0;
            for c in line.chars() {
                field.locations[(x, y)] = c.into();
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
