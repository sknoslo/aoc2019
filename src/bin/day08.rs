fn main() {
    let input = include_str!("../../input/08.txt").trim();

    let input = parse_input(&input);

    let p1 = part1(&input, 25, 6);

    println!("part 1: {}", p1);

    let p2 = part2(&input, 25, 6);

    println!("part 2:");
    println!("{}", p2);
}

fn part1(pixels: &Vec<u8>, width: usize, height: usize) -> usize {
    pixels
        .chunks(width * height)
        .map(|chunk| {
            chunk
                .iter()
                .fold((0, 0, 0), |(zeros, ones, twos), value| match value {
                    0 => (zeros + 1, ones, twos),
                    1 => (zeros, ones + 1, twos),
                    2 => (zeros, ones, twos + 1),
                    _ => unreachable!(),
                })
        })
        .min_by(|(a_zeros, _, _), (b_zeros, _, _)| a_zeros.cmp(&b_zeros))
        .map_or(0, |(_, ones, twos)| ones * twos)
}

fn part2(pixels: &Vec<u8>, width: usize, height: usize) -> String {
    // one char per pixel, plus an extra per line to account for the new line
    let mut picture = vec!['\n'; width * height + height];

    for chunk in pixels.chunks(width * height) {
        for (idx, pixel) in chunk.iter().enumerate() {
            let pixel_pos = get_pixel_pos(idx, width);

            picture[pixel_pos] = match (picture[pixel_pos], pixel) {
                ('0', _) => '0',
                ('1', _) => '1',
                ('2', value) | (_, value) => std::char::from_digit(*value as u32, 10).unwrap(),
            }
        }
    }

    picture.iter().collect()
}

// adjust final pixel position for the new line character
fn get_pixel_pos(index: usize, width: usize) -> usize {
    index + index / width
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "012211220012";

        let input = parse_input(&input);

        assert_eq!(part1(&input, 2, 2), 4);
    }

    #[test]
    fn part2_test() {
        let input = "0222112222120000";

        let input = parse_input(&input);

        assert_eq!(
            part2(&input, 2, 2),
            "\
01
10
"
        );
    }
}
