use station_plan::{StationVisit, schedule};

fn main() {
    let assignments = schedule(
        vec![
            StationVisit {
                id: "ST-10".into(),
                area: "North".into(),
                priority: 5,
                earliest_day: 1,
                latest_day: 5,
                duration_days: 1,
            },
            StationVisit {
                id: "ST-20".into(),
                area: "South".into(),
                priority: 8,
                earliest_day: 1,
                latest_day: 4,
                duration_days: 2,
            },
        ],
        1,
    )
    .unwrap();
    for item in assignments {
        println!(
            "{} crew {} days {}-{}",
            item.visit_id,
            item.crew + 1,
            item.start_day,
            item.end_day
        );
    }
}
