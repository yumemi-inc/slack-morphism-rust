//!
//! Support for Slack User Groups API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ratectl::SLACK_TIER2_METHOD_CONFIG;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use slack_morphism_models::*;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/usergroups.list
    ///
    pub async fn usergroups_list(
        &self,
        req: &SlackApiUserGroupsListRequest,
    ) -> ClientResult<SlackApiUserGroupsListResponse> {
        self.http_session_api
            .http_get(
                "usergroups.list",
                &vec![
                    (
                        "include_count",
                        req.include_count.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "include_disabled",
                        req.include_disabled.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "include_users",
                        req.include_users.map(|v| v.to_string()).as_ref(),
                    ),
                    ("team_id", req.team_id.as_ref().map(|x| x.value())),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsListRequest {
    include_count: Option<bool>,
    include_disabled: Option<bool>,
    include_users: Option<bool>,
    team_id: Option<SlackTeamId>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsListResponse {
    usergroups: Vec<SlackUserGroup>,
}
