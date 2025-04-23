use super::{AxumResponse, JsonResponse, Session};
use crate::{
    agents::{Agent, AgentRole},
    db::DbPool,
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub struct Role;

impl Role {
    pub async fn middleware(
        State(pool): State<DbPool>,
        req: Request,
        next: Next,
    ) -> Result<Response, AxumResponse<String>> {
        let headers = req.headers();
        let user_id = Session::extract_session_user_id(&headers);

        match Agent::find_by_user_id(&pool, &user_id) {
            Ok(agent) => match agent.role {
                AgentRole::Admin => Ok(next.run(req).await),
                AgentRole::Agent => {
                    let response = JsonResponse::send(403, None, None);
                    Err(response)
                }
            },
            _ => {
                let response = JsonResponse::send(403, None, None);
                Err(response)
            }
        }
    }
}
