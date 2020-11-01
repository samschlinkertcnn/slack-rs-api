//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::migration_types::*;

/// For Enterprise Grid workspaces, map local user IDs to global user IDs
///
/// Wraps https://api.slack.com/methods/migration.exchange

pub async fn exchange<R>(
    client: &R,
    request: &ExchangeRequest,
) -> Result<ExchangeResponse, ExchangeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
        request
            .to_old
            .as_ref()
            .map(|to_old| ("to_old", to_old.to_string())),
        Some(("users", request.users.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/migration.exchange");
    client
        .get(&url, &params[..])
        .await
        .map_err(ExchangeError::Client)
        .and_then(|result| {
            serde_json::from_str::<ExchangeResponse>(&result)
                .map_err(|e| ExchangeError::MalformedResponse(result, e))
        })
}