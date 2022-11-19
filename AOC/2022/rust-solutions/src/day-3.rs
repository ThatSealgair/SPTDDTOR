static INPUT_FILE: &'static str = include_str!("input.txt");
const BIT_LENGTH: usize = 12;

fn get_numbers() -> Vec<u16> {
    return INPUT_FILE
        .split_ascii_whitespace()
        .map(|str| u16::from_str_radix(str, 2).unwrap())
        .collect::<Vec<u16>>();
}

fn bit_matches(number: u16, position: usize, expected_bit: u16) -> bool {
    let mask = 1 << position;
    return ((number & mask) >> position) == (expected_bit as u16);
}


fn power_consumption() -> u64 {
    let numbers = get_numbers();
    let mask: u16 = 0x0FFF;
    let half_total = numbers.len() / 2;
    let mut counts: [usize; BIT_LENGTH] = [0; BIT_LENGTH];

    // tally up counts in each position
    for number in numbers {
        for position in 0..BIT_LENGTH {
            if bit_matches(number, position, 1) {
                counts[position] += 1;
            }
        }
    }

    // assemble back to an number
    let gamma: u16 = counts
        .iter()
        .enumerate()
        .fold(0, |accum, (position, count)| {
            let most_common_bit: u16 = if count > &half_total { 1 } else { 0 };
            return accum | (most_common_bit << position);
        });

    let epsilon = !gamma & mask;
    let multiplied = gamma as u64 * epsilon as u64;

    println!(
        "[Power Consumption]\n
        [Gamma] {}\n 
        [Epsilon] {}\n
        [Product] {}\n",
        gamma, epsilon, multiplied
    );

    return multiplied;
}

fn life_support() {
    enum BitSearch {
        LeastCommon,
        MostCommon,
    }

    fn compute_rating(search: BitSearch) -> u16 {
        let mut numbers = get_numbers();
        let mut counts: [usize; BIT_LENGTH] = [0; BIT_LENGTH];
        let mut position = BIT_LENGTH;

        while numbers.len() > 1 {
            position -= 1;
            let mut current_total = numbers.len();
            let half_total = current_total as f32 / 2.0;

            for number in &numbers {
                if bit_matches(*number, position, 1) {
                    counts[position] += 1;
                }
            }

            let count = counts[position];
            let common_bit = match search {
                BitSearch::MostCommon => {
                    if (count as f32) >= half_total {
                        1
                    } 
                    else {
                        0
                    }
                }
                BitSearch::LeastCommon => {
                    if (count as f32) < half_total {
                        1
                    } 
                    else {
                        0
                    }
                }
            };

            numbers.retain(|number| {
                let matched = bit_matches(*number, position, common_bit);

                if !matched {
                    current_total -= 1;
                }

                return matched || current_total == 0;
            });
        }

        assert_eq!(numbers.len(), 1);
        return numbers[0];
    }

    let oxygen_generator_rating = compute_rating(BitSearch::MostCommon);
    let co2_scrubber_rating = compute_rating(BitSearch::LeastCommon);

    println!(
        "[Life Support]\n 
        [Oxygen Generator] {}\n 
        [CO2 Scrubber] {}\n 
        [Product] {}",
        oxygen_generator_rating, 
        co2_scrubber_rating, 
        oxygen_generator_rating as u64 * co2_scrubber_rating as u64
    );
}

fn main() {
    power_consumption();
    life_support();
}
