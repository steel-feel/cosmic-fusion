#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::helpers::CwTemplateContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, PullFundsMsg};
    use crate::state::Timelocks;
    use cosmwasm_std::testing::MockApi;
    use cosmwasm_std::{
        coins, instantiate2_address, Addr, AnyMsg, Api, Coin, CosmosMsg, Empty, Uint128,
    };
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use injective_std::shim::Any;
    use injective_std::types::cosmos::authz::v1beta1::{Grant, MsgGrant};
    use injective_std::types::cosmos::bank::v1beta1::SendAuthorization;
    use injective_std::types::cosmos::base::v1beta1::Coin as InjectiveCoin;
    use prost::Message;
    use sha3::{Digest, Keccak256};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const MAKER: &str = "MAKER";
    const TAKER: &str = "TAKER";
    const NATIVE_DENOM: &str = "stake";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &MockApi::default().addr_make(TAKER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let maker = app.api().addr_make(MAKER);
        let taker = app.api().addr_make(TAKER);

        let cw_template_id = app.store_code_with_creator(maker.clone(), contract_template());

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &maker, coins(100, "stake"))
                .unwrap();
        });

        assert_eq!(
            app.wrap()
                .query_balance(&maker, NATIVE_DENOM)
                .unwrap()
                .amount,
            Uint128::new(100)
        );

        let order_hash = {
            let mut hasher = Keccak256::new();
            hasher.update(b"orderhash");
            hex::encode(&hasher.finalize()) //.to_ascii_lowercase()
        };
      
        // let code_info_response = app.wrap().query_wasm_code_info(cw_template_id).unwrap();

        // let contract_addr = instantiate2_address(
        //     code_info_response.checksum.as_slice(),
        //     &app.api().addr_canonicalize(maker.as_str()).unwrap(),
        //     order_hash.as_bytes(),
        // )
        // .unwrap();

        let hashlock = {
            let mut hasher = Keccak256::new();
            hasher.update(b"secret");
            hex::encode(&hasher.finalize())
        };
        //Initiate
        let msg = InstantiateMsg {
            rescue_delay: 1,
            hashlock,
            order_hash,
            maker: Addr::unchecked("maker"),
            taker: Addr::unchecked("taker"),
            timelocks: Timelocks {
                withdrawal: 1000,
                public_withdrawal: 2000,
                dest_cancellation: 3000,
                src_cancellation: 3000,
                src_withdrawal: 5000,
            },
            token: Coin::new(1000u32, "stake"),
        };

        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(TAKER),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();
        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

   /*
    *
    *Note: Can not do integration testing since
    * MsgAuthz Bank SendAuthorization is not supported yet by multi-test
    *
    */
    //     let bank_spend_auth =  SendAuthorization{
    //         allow_list: vec![maker.to_string()],
    //         spend_limit: vec![InjectiveCoin {
    //             amount: String::from("1"),
    //             denom: String::from("stake"),
    //         }],
    //     };

    //     let authorization = Any {
    //         type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
    //         value: bank_spend_auth.encode_to_vec(),
    //     };

    //     let msg_grant = MsgGrant {
    //         granter: maker.to_string(),
    //         grantee: cw_template_contract_addr.to_string(),
    //         grant: Some(Grant {
    //             expiration: None,
    //             authorization: Some(authorization),
    //         }),
    //     };

    //     let cosmos_msg_grant = CosmosMsg::Any(AnyMsg {
    //         type_url: "/cosmos.authz.v1beta1.MsgGrant".to_string(),
    //         value: msg_grant.into(),
    //     });

    //    let err = app.execute(maker.clone(), cosmos_msg_grant).unwrap_err();

    //     println!(" hellor there {:?}" , err.to_string() );

      

    //     let pull_funds_msg = ExecuteMsg::PullFunds(PullFundsMsg {
    //         from: maker,
    //         amount: Coin {
    //             denom: "stake".to_string(),
    //             amount: Uint128::from_str("1").unwrap(),
    //         },
    //     });

    //     let cw_msg = cw_template_contract.call(pull_funds_msg).unwrap();

        // app.execute(taker, cw_msg);

        // app.router().querier(, storage, block_info)

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn initiated_successfully() {
            let (mut app, cw_template_contract) = proper_instantiate();
            assert!(true);
            // let msg = ExecuteMsg::Increment {};
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
