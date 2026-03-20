use serde::Deserialize;

#[derive(Deserialize)]
pub struct MeResponse {
    pub data: User,
}

#[derive(Deserialize)]
pub struct PresencesResponse {
    pub data: PresencesResponseData,
}

#[derive(Deserialize)]
pub struct PresencesResponseData {
    pub users: Vec<UserPresence>,
    pub teams: Vec<EntityReference>,
}

pub struct AggregatedMeResponse {
    pub user: User,
    pub company: Company,
    pub worktime_regulation: WorktimeRegulation,
}

#[derive(Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub weekstart_monday: bool,
    pub weekend_friday: bool,
    pub active: bool,
    pub timeformat_12h: bool,
    pub language: String,
    pub timezone: String,
    pub teams_id: i32,
    pub initials: String,
    pub nonbusiness_groups_id: i32,
    pub nonbusinessgroups_id: i32,
    pub number: Option<String>,
    pub role: String,
    pub work_time_regulations_id: Option<i32>,
    pub default_work_time_regulation: bool,
    pub boss: i32,
    pub absence_managers_id: Vec<i32>,
    pub email: String,
    pub can_generally_see_absences: bool,
    pub can_generally_manage_absences: bool,
    pub can_add_customers: bool,
    pub default_holidays_count: bool,
    pub default_target_hours: bool,
    pub future_coworker: bool,
    pub start_date: String,
    pub wage_type: i32,
    pub creator: i32,
    pub created_at: String,
    pub worktime_regulation_id: Option<i32>,
    pub edit_lock: Option<String>,
    pub edit_lock_dyn: Option<i32>,
    pub edit_lock_sync: bool,
    pub work_time_edit_lock_days: i32,
    pub access_groups_ids: Vec<i32>,
    pub support_pin: String,
}

#[derive(Deserialize, PartialEq)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub timezone_default: String,
    pub currency: String,
    pub allow_entries_text_multiline: bool,
    pub allow_entries_for_customers: bool,
    pub allow_entry_overlaps: bool,
    pub force_linked_entry_times: bool,
    pub default_customers_id: i32,
    pub default_services_id: i32,
    pub module_absence: bool,
    pub module_work_time: bool,
    pub module_targethours: bool,
    pub module_target_hours: bool,
    pub module_userreports: bool,
    pub module_user_reports: bool,
    pub module_project_times: bool,
    pub module_entries_texts: bool,
    pub nonbusiness_group_default: i32,
    pub onboarding_complete: bool,
    pub worktime_regulation_default: i32,
    pub worktime_evaluate_regulations_since: String,
    pub worktime_force_breaks: i32,
    pub holidays_count_default: f64,
    pub absence_reduces_target_hours: bool,
    pub compensate_day_default: f64,
    pub compensate_month_default: f64,
    pub target_hours_default: serde_json::Value,
}

#[derive(Deserialize, PartialEq)]
pub struct WorktimeRegulation {
    pub id: i32,
    pub name: String,
    pub add_to_worktime: bool,
    pub daily_max: i32,
    pub weekly_max: i32,
    pub interval_max: i32,
}

#[derive(Deserialize, PartialEq)]
pub struct TimeEntry {
    pub id: i32,
    pub customers_id: i32,
    pub projects_id: i32,
    pub subprojects_id: i32,
    pub users_id: i32,
    pub billable: i32,
    pub texts_id: i32,
    pub text: String,
    pub time_since: String,
    pub time_until: String,
    pub time_insert: String,
    pub time_last_change: String,
    pub test_data: bool,
    pub customers_name: String,
    pub projects_name: String,
    pub subprojects_name: String,
    pub users_name: String,
    pub revenue: f64,
    #[serde(rename = "type")]
    pub entry_type: i32,
    pub services_id: i32,
    pub duration: i32,
    pub time_last_change_work_time: String,
    pub time_clocked_since: String,
    pub clocked: bool,
    pub clocked_offline: bool,
    pub hourly_rate: f64,
    pub services_name: String,
}

#[derive(Deserialize, PartialEq)]
pub struct AccessGroup {
    pub id: i32,
    pub name: String,
    pub users_ids: Vec<i32>,
    pub has_elevated_access: Option<bool>,
    pub has_master_data_access: Option<bool>,
    pub company_default: bool,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub items_per_page: i32,
    pub current_page: i32,
    pub count_pages: i32,
    pub count_items: i32,
}

#[derive(Deserialize, PartialEq)]
pub struct Service {
    pub id: i32,
    pub name: String, // interne Arbeitszeit...
    pub number: Option<String>,
    pub active: bool,
}

#[derive(Deserialize, PartialEq)]
pub struct Project {
    pub id: i32,
    pub customers_id: i32,
    pub name: String,
    pub number: String,
    pub active: bool,
    pub billable_default: bool,
    pub note: String,
    pub billed_money: f64,
    pub billed_completely: bool,
    pub completed: bool,
    pub completed_at: Option<String>,
    pub revenue_factor: f64,
    pub deadline: Option<String>,
    pub start_date: Option<String>,
    pub automatic_completion: bool,
    pub test_data: bool,
    pub count_subprojects: i32,
    pub budget: serde_json::Value,
    pub bill_service_id: Option<String>,
    pub service_assignments: Vec<serde_json::Value>,
}

#[derive(Deserialize, PartialEq)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub leader: i32,
    pub user_ids: Vec<i32>,
}

#[derive(Deserialize)]
pub struct UserFilter {
    pub active: bool,
    pub fulltext: Option<String>,
    pub teams_id: Option<Vec<i32>>,
}

// Holy cow... what in heavens name is this format? Used for query parameters...
// TODO: use a custom builder later on for such "things"
impl ToString for UserFilter {
    fn to_string(&self) -> String {
        let mut query = String::new();
        if let Some(teams_id) = &self.teams_id {
            let teams_str: Vec<String> = teams_id.iter().map(|id| id.to_string()).collect();
            query.push_str("filter[teams_id][]=");
            query.push_str(&teams_str.join("&filter[teams_id][]="));
        }
        if self.active {
            query.push_str("&filter[active]=true");
        }
        if let Some(fulltext) = &self.fulltext {
            query.push_str(&format!("&filter[fulltext]={}", fulltext));
        }
        query
    }
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct EntityReference {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct RunningClock {
    pub id: i32,
    pub time_since: String,
    pub customer: Option<EntityReference>,
    pub project: Option<EntityReference>,
    pub subproject: Option<EntityReference>,
    pub service: Option<EntityReference>,
    pub text: Option<String>,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct UserAbsence {
    pub id: i32,
    pub users_id: i32,
    pub date_since: String,
    pub date_until: String,
    pub status: i32,
    pub public_note: Option<String>,
    pub count_days: f64,
    #[serde(rename = "type")]
    pub absence_type: Option<i32>,
    pub note: Option<String>,
    pub sick_note: Option<bool>,
    pub count_hours: Option<f64>,
    pub date_enquired: Option<String>,
    pub date_approved: Option<String>,
    pub approved_by: Option<i32>,
}

// Inofficial from web api/v2/users/presences
#[derive(Deserialize, PartialEq, Clone)]
pub struct UserPresence {
    pub id: i32,
    pub name: String,
    pub can_see_timetable: bool,
    pub teams_id: i32,
    pub is_work_day: bool,
    pub nonbusiness_day: Option<serde_json::Value>,
    pub absences: Vec<UserAbsence>,
    pub running_clock: Option<RunningClock>,
    pub can_manage_absences: bool,
}

impl Into<(String, UserPresence)> for UserPresence {
    fn into(self) -> (String, UserPresence) {
        (self.name.clone(), self)
    }
}
