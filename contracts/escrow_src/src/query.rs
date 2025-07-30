use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// InjectiveRoute is enum type to represent injective query route path
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CosmicRoute {
    Authz,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CosmicQueryWrapper {
    pub route: CosmicRoute,
    pub query_data: CosmicQuery,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CosmicQuery {
    Authz { }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CosmicMsgWrapper {
    pub route: CosmicRoute,
    // pub msg_data: InjectiveMsg,
}

impl From<CosmicMsgWrapper> for CosmosMsg<CosmicMsgWrapper> {
    fn from(s: CosmicMsgWrapper) -> CosmosMsg<CosmicMsgWrapper> {
        CosmosMsg::Custom(s)
    }
}

impl CustomMsg for CosmicMsgWrapper {}

