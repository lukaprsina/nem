use std::fs;

use color_eyre::Result;
use itertools::Itertools;
use rand::Rng;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// Spraševalnik nemščine
fn main() -> Result<()> {
    color_eyre::install()?;

    let contents = fs::read_to_string("test.txt")?;
    let blocks = contents
        .split(&LINE_ENDING.repeat(2))
        .map(|block| block.lines().collect_vec())
        .collect_vec();

    let mut names = blocks
        .iter()
        .map(|block| {
            *block
                .first()
                .expect("There is no elements in this block of text")
        })
        .collect_vec();

    names.push("MIXED BAG");

    let select = inquire::Select::new("Choose a category", names).prompt()?;

    if select == "MIXED BAG" {
        let questions = blocks
            .into_iter()
            .map(|mut block| {
                block.remove(0);
                block
            })
            .flatten()
            .collect_vec();

        ask_questions(questions)?;
    } else {
        for mut block in blocks {
            let first = block.remove(0);

            if first == select {
                ask_questions(block)?;
            }
        }
    }

    Ok(())
}

fn ask_questions(lines: Vec<&str>) -> Result<()> {
    'outer: loop {
        let mut current = lines.clone();

        'inner: loop {
            if current.len() == 0 {
                break 'inner;
            }

            let index = rand::thread_rng().gen_range(0..current.len());
            let line = current.remove(index);

            match line.split_once("-") {
                Some((german, slovenian)) => {
                    println!("\n\n{slovenian}");
                    let prompt = inquire::Confirm::new("").with_default(true).prompt()?;
                    if !prompt {
                        break 'outer;
                    }
                    println!("{german}");
                }
                None => println!("Can't parse: \"{}\"", line),
            }
        }
    }

    Ok(())
}
