#[must_use]
pub fn calc_a(input: &str) -> String {
    get_first_marker_pos(input, 4)
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    get_first_marker_pos(input, 14)
}

fn get_first_marker_pos(input: &str, marker_size: usize) -> String {
    let (_, idxs) = input.chars().enumerate().fold(
        (Vec::with_capacity(marker_size), Vec::new()),
        |(mut acc, mut idxs), (i, c)| {
            if let Some(idx) = acc.iter().position(|c2| *c2 == c) {
                if idx == acc.len() - 1 {
                    acc.truncate(0);
                } else {
                    acc = acc.split_off(idx + 1);
                }
            }
            acc.push(c);
            if acc.len() == marker_size {
                idxs.push(i + 1);
                acc.truncate(0);
            }

            (acc, idxs)
        },
    );

    idxs.first()
        .map_or_else(|| "No marker found".to_string(), ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_b() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = "19";
        let actual = calc_b(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calc_a() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = "7";
        let actual = calc_a(input);
        assert_eq!(actual, expected);
    }
}
