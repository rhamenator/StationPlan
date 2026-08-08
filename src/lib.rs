#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StationVisit {
    pub id: String,
    pub area: String,
    pub priority: u8,
    pub earliest_day: u32,
    pub latest_day: u32,
    pub duration_days: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub visit_id: String,
    pub crew: usize,
    pub start_day: u32,
    pub end_day: u32,
}

pub fn schedule(
    mut visits: Vec<StationVisit>,
    crew_count: usize,
) -> Result<Vec<Assignment>, String> {
    if crew_count == 0 {
        return Err("at least one crew is required".into());
    }
    for visit in &visits {
        if visit.id.trim().is_empty()
            || visit.duration_days == 0
            || visit.earliest_day > visit.latest_day
        {
            return Err(format!("invalid visit {}", visit.id));
        }
    }
    visits.sort_by_key(|visit| {
        (
            std::cmp::Reverse(visit.priority),
            visit.latest_day,
            visit.earliest_day,
            visit.id.clone(),
        )
    });
    let mut next_days = vec![0; crew_count];
    let mut assignments = Vec::new();
    for visit in visits {
        let (crew, start_day) = next_days
            .iter()
            .enumerate()
            .map(|(crew, available)| (crew, (*available).max(visit.earliest_day)))
            .min_by_key(|(crew, start)| (*start, *crew))
            .unwrap();
        let end_day = start_day + visit.duration_days - 1;
        if end_day > visit.latest_day {
            return Err(format!(
                "visit {} cannot fit its scheduling window",
                visit.id
            ));
        }
        next_days[crew] = end_day + 1;
        assignments.push(Assignment {
            visit_id: visit.id,
            crew,
            start_day,
            end_day,
        });
    }
    Ok(assignments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_priority_visit_is_assigned_first_without_crew_overlap() {
        let result = schedule(
            vec![
                StationVisit {
                    id: "low".into(),
                    area: "A".into(),
                    priority: 1,
                    earliest_day: 1,
                    latest_day: 5,
                    duration_days: 2,
                },
                StationVisit {
                    id: "high".into(),
                    area: "B".into(),
                    priority: 9,
                    earliest_day: 1,
                    latest_day: 3,
                    duration_days: 2,
                },
            ],
            1,
        )
        .unwrap();
        assert_eq!(result[0].visit_id, "high");
        assert_eq!(result[1].start_day, 3);
    }
}
