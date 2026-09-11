use crate::http::SamResponse;

pub struct DashboardParser;

impl DashboardParser {
    pub const fn parse_response(response: &SamResponse) -> DashboardResponse {
        if response.status != 200 {
            return DashboardResponse::Unauthenticated;
        }

        DashboardResponse::Accessed
    }
}

pub enum DashboardResponse {
    Accessed,
    Unauthenticated,
}
