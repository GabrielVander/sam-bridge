use crate::http::SamResponse;

pub const fn parse_dashboard_response(response: &SamResponse) -> DashboardResponse {
    if response.status != 200 {
        return DashboardResponse::Unauthenticated;
    }

    DashboardResponse::Accessed
}

pub enum DashboardResponse {
    Accessed,
    Unauthenticated,
}
