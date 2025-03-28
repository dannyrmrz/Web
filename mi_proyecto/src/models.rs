use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Match {
    pub id: i32,
    #[serde(rename = "homeTeam")]
    pub equipo_local: String,
    #[serde(rename = "awayTeam")]
    pub equipo_visitante: String,
    #[serde(rename = "matchDate")]
    pub date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMatch {
    #[serde(rename = "homeTeam")]
    pub home_team: String,
    #[serde(rename = "awayTeam")]
    pub away_team: String,
    #[serde(rename = "matchDate")]
    pub match_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMatch {
    #[serde(rename = "homeTeam")]
    pub home_team: Option<String>,
    #[serde(rename = "awayTeam")]
    pub away_team: Option<String>,
    #[serde(rename = "matchDate")]
    pub match_date: Option<NaiveDateTime>,
}