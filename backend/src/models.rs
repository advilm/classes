use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config<'a> {
    pub postgres_url: &'a str,
    pub port: String,
}

// Term search
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub code: String,
    pub description: String,
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
// Course page data
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub success: bool,
    pub total_count: i64,
    pub data: Vec<Daum>,
    pub page_offset: i64,
    pub page_max_size: i64,
    pub sections_fetched_count: i64,
    pub path_mode: String,
    pub search_results_configs: Value,
    pub ztc_encoded_image: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub campus_description: String,
    pub course_number: String,
    pub course_reference_number: String,
    pub course_title: String,
    pub credit_hour_high: Option<f64>,
    pub credit_hour_indicator: Option<String>,
    pub credit_hour_low: f64,
    pub credit_hours: Option<f64>,
    pub cross_list: Option<String>,
    pub cross_list_available: Option<i64>,
    pub cross_list_capacity: Option<i64>,
    pub cross_list_count: Option<i64>,
    pub enrollment: i64,
    pub faculty: Vec<Faculty>,
    pub id: i64,
    pub instructional_method: String,
    pub instructional_method_description: String,
    pub is_section_linked: bool,
    pub link_identifier: Option<String>,
    pub maximum_enrollment: i64,
    pub meetings_faculty: Vec<MeetingsFaculty>,
    pub open_section: bool,
    pub part_of_term: String,
    pub reserved_seat_summary: Option<ReservedSeatSummary>,
    pub schedule_type_description: String,
    pub seats_available: i64,
    pub section_attributes: Value,
    pub sequence_number: String,
    pub subject: String,
    pub subject_course: String,
    pub subject_description: String,
    pub term: String,
    pub term_desc: String,
    pub wait_available: i64,
    pub wait_capacity: i64,
    pub wait_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faculty {
    pub banner_id: String,
    pub category: Value,
    pub class: String,
    pub course_reference_number: String,
    pub display_name: String,
    pub email_address: Option<String>,
    pub primary_indicator: bool,
    pub term: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingsFaculty {
    pub category: String,
    pub class: String,
    pub course_reference_number: String,
    pub faculty: Vec<Value>,
    pub meeting_time: MeetingTime,
    pub term: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingTime {
    pub begin_time: Option<String>,
    pub building: Option<String>,
    pub building_description: Option<String>,
    pub campus: Option<String>,
    pub campus_description: Option<String>,
    pub category: String,
    pub class: String,
    pub course_reference_number: String,
    pub credit_hour_session: f64,
    pub end_date: String,
    pub end_time: Option<String>,
    pub friday: bool,
    pub hours_week: f64,
    pub meeting_schedule_type: String,
    pub meeting_type: String,
    pub meeting_type_description: String,
    pub monday: bool,
    pub room: Option<String>,
    pub saturday: bool,
    pub start_date: String,
    pub sunday: bool,
    pub term: String,
    pub thursday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservedSeatSummary {
    pub class: String,
    pub course_reference_number: String,
    pub maximum_enrollment_reserved: i64,
    pub maximum_enrollment_unreserved: i64,
    pub seats_available_reserved: i64,
    pub seats_available_unreserved: i64,
    pub term_code: String,
    pub wait_available_reserved: i64,
    pub wait_available_unreserved: i64,
    pub wait_capacity_reserved: i64,
    pub wait_capacity_unreserved: i64,
}
