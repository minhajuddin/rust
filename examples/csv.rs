struct Penguin {
    name: String,
    height: u8,
}

fn parse_penguin(record: &str) -> Option<Penguin> {
    let cols: Vec<_> = record.split(",").map(|c| c.trim()).collect();

    if cols.len() != 2 {
        return None;
    }

    match cols[1].parse() {
        Ok(height) => Some(Penguin {
            name: cols[0].into(),
            height,
        }),
        _ => None,
    }
}

pub fn main() {
    let penguin_data = "\
      common name,length (cm)
      Little penguin,33
      Yellow-eyed penguin,65
      Fiordland penguin,60
      Invalid,data
      ";

    let penguins = penguin_data
                            .lines()
                            .filter_map(parse_penguin);

    for (i, penguin) in penguins.enumerate() {
        println!(
            "#{row_id} name: {name}, height: {height}cm",
            row_id = i + 1,
            name = penguin.name,
            height = penguin.height
        )
    }
}
