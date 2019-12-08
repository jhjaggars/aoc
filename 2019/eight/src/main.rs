use std::fs;
use std::iter::FromIterator;

fn main() {
    let image = fs::read_to_string("input.txt").unwrap();
    let width = 25;
    let height = 6;
    let layer_length = width * height;
    let mut lvec: Vec<Vec<_>> = vec![];
    for offset in 0..image.len() - 1 {
        if offset % layer_length != 0 {
            continue;
        }
        lvec.push(
            image
                .trim()
                .chars()
                .skip(offset)
                .take(layer_length)
                .collect(),
        );
    }
    let mut b: Vec<_> = lvec.pop().unwrap();
    lvec.reverse();

    for layer in lvec.iter() {
        for (i, ch) in layer.iter().enumerate() {
            if *ch == '2' {
                continue;
            }
            b[i] = *ch;
        }
    }

    for row in 0..height {
        let line: Vec<_> = b
            .iter()
            .skip(row * width)
            .take(width)
            .map(|ch| match ch {
                '0' => ' ',
                '1' => '@',
                '2' => ' ',
                _ => unreachable!("ut oh"),
            })
            .collect();
        println!("{:?}", String::from_iter(line));
    }
}
