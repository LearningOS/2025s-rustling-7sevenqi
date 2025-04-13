// hashmaps3.rs

use std::collections::HashMap;

struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        // 更新队伍1的统计数据
        let team1 = scores.entry(team_1_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team1.goals_scored += team_1_score;
        team1.goals_conceded += team_2_score;

        // 更新队伍2的统计数据
        let team2 = scores.entry(team_2_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team2.goals_scored += team_2_score;
        team2.goals_conceded += team_1_score;
    }
    scores
}
