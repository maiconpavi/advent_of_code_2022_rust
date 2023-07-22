const fn main() {}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    type DayFn = Box<dyn Fn(&str) -> String>;

    struct Day {
        a: DayFn,
        b: Option<DayFn>,
    }

    impl<F: Fn(&str) -> String + 'static> From<F> for Day {
        fn from(a: F) -> Self {
            Self {
                a: Box::new(a),
                b: None,
            }
        }
    }

    impl<F1, F2> From<(F1, F2)> for Day
    where
        F1: Fn(&str) -> String + 'static,
        F2: Fn(&str) -> String + 'static,
    {
        fn from((a, b): (F1, F2)) -> Self {
            Self {
                a: Box::new(a),
                b: Some(Box::new(b)),
            }
        }
    }

    fn test_day_input<F, P>(f: F, path: P, extra: &str)
    where
        F: Fn(&str) -> String,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let folder = path
            .parent()
            .expect("test case must be in a directory")
            .file_stem()
            .expect("test case parent folder must have a file stem")
            .to_string_lossy()
            .to_string();
        let suffix = if path.ends_with("test_input.txt") {
            "_test"
        } else {
            ""
        };
        let input = std::fs::read_to_string(path).expect("failed to read input");
        if input.is_empty() {
            return;
        }

        let mut settings = insta::Settings::new();
        settings.set_snapshot_suffix(folder + extra + suffix);

        settings.bind(|| {
            let result = f(&input);
            insta::assert_snapshot!(result);
        });
    }

    fn test_day(d: Day, day: usize) {
        let path: PathBuf = format!("inputs/day_{day:02}").into();
        test_day_input(&d.a, path.join("test_input.txt"), "a");
        test_day_input(&d.a, path.join("input.txt"), "a");

        if let Some(b) = d.b {
            test_day_input(&b, path.join("test_input.txt"), "b");
            test_day_input(&b, path.join("input.txt"), "b");
        }
    }

    #[test]
    fn test_days() {
        let days = [
            (day_01::calc_a, day_01::calc_b).into(),
            (day_02::calc_a, day_02::calc_b).into(),
            (day_03::calc_a, day_03::calc_b).into(),
            (day_04::calc_a, day_04::calc_b).into(),
            (day_05::calc_a, day_05::calc_b).into(),
            // (day_06::calc_a).into(),
        ];
        for (day, functions) in days.into_iter().enumerate() {
            test_day(functions, day + 1);
        }
    }
}
