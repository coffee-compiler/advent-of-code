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

    let result = count_safe_reports(&reports);

    println!("{}", result);
}

fn count_safe_reports(reports: &[&[u32]]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count()
}

fn is_safe_report(report: &[u32]) -> bool {
    const MAX_DISTANCE: u32 = 3;
    let mut asc = true;

    for i in 0..report.len() - 1 {
        let n1 = report[i];
        let n2 = report[i + 1];

        if i == 0 {
            asc = n1 < n2;
        }

        if asc != (n1 < n2) || n1 == n2 || n1.abs_diff(n2) > MAX_DISTANCE {
            return false;
        }
    }

    true
}
