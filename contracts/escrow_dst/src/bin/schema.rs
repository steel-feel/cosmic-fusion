use cosmwasm_schema::write_api;

use escrow_dst::msg::{ExecuteMsg, InstantiateMsgData, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsgData,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
