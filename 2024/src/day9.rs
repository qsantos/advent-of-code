use std::fmt::Display;

fn file_hash(position: u64, file_index: u64, file_size: u64) -> u64 {
    // a1 = position * file_index
    // an = (position + file_size - 1) * file_index
    // n = file_size
    // n * (a1 + an) / 2
    file_size * file_index * (2 * position + file_size - 1) / 2
}

pub fn part1(input: &str) -> impl Display {
    let numbers: Vec<u64> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| c as u64))
        .collect();
    assert_ne!(numbers.len() % 2, 0);
    let mut res = 0;
    let mut forward_index = 0;
    let mut backward_index = numbers.len() - 1;
    let mut remaining = numbers[backward_index];
    let mut position = 0;
    while forward_index < backward_index {
        // occupied blocks
        let file_size = numbers[forward_index];
        let file_index = forward_index / 2;
        for _ in 0..file_size {
            res += position * file_index;
            position += 1;
        }
        forward_index += 1;
        // free space
        let mut free_size = numbers[forward_index];
        while free_size > 0 {
            if remaining == 0 {
                backward_index -= 2;
                if forward_index >= backward_index {
                    break;
                }
                remaining = numbers[backward_index];
            }
            let usable = free_size.min(remaining);
            res += file_hash(position as u64, backward_index as u64 / 2, usable) as usize;
            position += usable as usize;
            free_size -= usable;
            remaining -= usable;
        }
        forward_index += 1;
    }
    res += file_hash(position as u64, backward_index as u64 / 2, remaining) as usize;
    res
}

pub fn part2(input: &str) -> impl Display {
    let numbers: Vec<u64> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| c as u64))
        .collect();

    // locate files and free spaces
    struct File {
        index: u64,
        position: u64,
        size: u64,
    }
    struct FreeSpace {
        position: u64,
        size: u64,
    }
    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut position = 0;
    for i in (0..numbers.len() - 1).step_by(2) {
        // file
        let size = numbers[i];
        files.push(File {
            index: (i / 2) as u64,
            position,
            size,
        });
        position += size;
        // free space
        let size = numbers[i + 1];
        free_spaces.push(FreeSpace { position, size });
        position += size;
    }
    // last file
    let size = numbers[numbers.len() - 1];
    files.push(File {
        index: (numbers.len() / 2) as u64,
        position,
        size,
    });

    // iterate over files in reverse order
    let mut res = 0;
    for file in files.into_iter().rev() {
        // try to find a free space that is large enough for the file
        let free_space = free_spaces
            .iter()
            .filter(|fs| fs.position < file.position)
            .position(|fs| fs.size >= file.size);
        if let Some(free_space_index) = free_space {
            // move the file to the free space and update checksum
            res += file_hash(
                free_spaces[free_space_index].position,
                file.index,
                file.size,
            );
            // update the free space
            free_spaces[free_space_index].size -= file.size;
            free_spaces[free_space_index].position += file.size;
            if free_spaces[free_space_index].size == 0 {
                free_spaces.remove(free_space_index);
            }
        } else {
            // keep the file at its position and update checksum
            res += file_hash(file.position, file.index, file.size);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day9.txt");
    const INPUT: &str = include_str!("../inputs/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "1928");
        assert_eq!(part1(INPUT).to_string(), "6310675819476");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "2858");
        assert_eq!(part2(INPUT).to_string(), "6335972980679");
    }
}
