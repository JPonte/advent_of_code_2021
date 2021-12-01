use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    for day in 2..25 {
        let main_text = format!("use utils::*;\n\nfn main() {{\n   let file = read_file(\"inputs/day_{:02}/input.txt\");\n}}", day);
        std::fs::create_dir(format!("src/day_{:02}", day))?;
        let mut file = File::create(format!("src/day_{:02}/main.rs", day))?;
        file.write_all(main_text.as_bytes())?;
        std::fs::create_dir(format!("inputs/day_{:02}", day))?;
    }

    let cargo_str: String = (2..25)
        .map(|day| {
            format!(
                "[[bin]]\nname = \"day_{:02}\"\npath = \"src/day_{:02}/main.rs\"\n\n",
                day, day
            )
        })
        .collect();
    let mut cargo_file = File::create("cargo_bins.toml")?;
    cargo_file.write_all(cargo_str.as_bytes())?;

    Ok(())
}
