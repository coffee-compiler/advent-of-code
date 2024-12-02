use std::fs;

fn main() {
    let reports: Vec<Vec<u32>> = fs::read_to_string("assets/day_2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let reports: Vec<&[u32]> = reports.iter().map(|report| report.as_slice()).collect();

    let result_p1 = count_safe_reports(&reports, false);
    let result_p2 = count_safe_reports(&reports, true);

    println!("{}", result_p1);
    println!("{}", result_p2);
}

fn count_safe_reports(reports: &[&[u32]], dampened: bool) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report, dampened))
        .count()
}

fn is_safe_report(report: &[u32], dampened: bool) -> bool {
    const MAX_DISTANCE: u32 = 3;
    let mut bad_levels: u32 = 0;
    let mut asc = true;

    for i in 0..report.len() - 1 {
        let n1 = report[i];
        let n2 = report[i + 1];

        if i == 0 {
            asc = n1 < n2;
        }

        if asc != (n1 < n2) || n1 == n2 || n1.abs_diff(n2) > MAX_DISTANCE {
            bad_levels += 1;
        }

        if (bad_levels > 0 && !dampened) || (bad_levels > 1 && dampened) {
            return false;
        }
    }

    true
}
