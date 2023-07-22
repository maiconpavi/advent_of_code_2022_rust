#[must_use]
pub fn calc_a(input: &str) -> String {
    get_calorioes(input)
        .max()
        .expect("No calories found")
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let mut calories = get_calorioes(input).collect::<Vec<_>>();
    calories.sort_unstable();
    calories.reverse();

    calories[..3].iter().sum::<u64>().to_string()
}

fn get_calorioes(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split("\n\n").map(|calories_raw| {
        calories_raw
            .split('\n')
            .filter_map(|calorie| calorie.parse::<u64>().ok())
            .sum::<u64>()
    })
}
