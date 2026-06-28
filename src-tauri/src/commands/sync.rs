// use serde::Serialize;
// use tauri::State;

// use crate::{
//     infrastructure::database::connection::get_connection,
//     services::sync_service,
//     state::AppState,
// };

// #[derive(Debug, Serialize)]
// pub struct CalculatedMetricsResponse {
//     pub total_commits: u32,
//     pub weekly_commits: u32,
//     pub unique_contributors: u32,
//     pub health_score: f32,
//     pub activity_level: String,
// }

// #[tauri::command]
// pub fn calculate_repository_metrics(
//     repo_id: i64,
//     state: State<'_, AppState>,
// ) -> Result<CalculatedMetricsResponse, String> {
//     let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
//     sync_service::calculate_repository_metrics(&conn, repo_id)
//         .map(|metrics| CalculatedMetricsResponse {
//             total_commits: metrics.total_commits,
//             weekly_commits: metrics.weekly_commits,
//             unique_contributors: metrics.unique_contributors,
//             health_score: metrics.health_score.value(),
//             activity_level: format!("{:?}", metrics.activity_level),
//         })
//         .map_err(|e| e.to_string())
// }
