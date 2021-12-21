use crate::msg::{FoodInitMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, State};
use cosmwasm_std::{
    Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};
use secret_toolkit::utils::InitCallback;

impl InitCallback for FoodInitMsg {
    const BLOCK_SIZE: usize = 256;
}

const CODE_ID: u64 = 1;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State { food_token: None };

    config(&mut deps.storage).save(&state)?;

    // TODO: Init parameters omitted
    let food_init_msg = FoodInitMsg {
        name: "Food".to_string(),
        admin: Some(env.contract.address),
        symbol: "FOOD".to_string(),
        decimals: 2,
        initial_balances: None,
        prng_seed: Binary::from(b"prng_seed"),
        config: None,
    };

    let cosmos_msg =
        food_init_msg.to_cosmos_msg("LABEL".to_string(), CODE_ID, "CODE_HASH".to_string(), None)?;

    Ok(InitResponse {
        messages: vec![cosmos_msg],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Register { snip_20 } => try_register(deps, env, snip_20),
    }
}

pub fn try_register<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    snip_20: CanonicalAddr,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        if state.food_token == None {
            state.food_token = Some(snip_20);
            Ok(state)
        } else {
            Err(StdError::GenericErr {
                msg: "Already registered".to_string(),
                backtrace: None,
            })
        }
    })?;
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(0, &[]);

        let msg = InitMsg {};
        let env = mock_env("sender", &coins(0, "f"));
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn register() {
        let mut deps = mock_dependencies(0, &[]);

        let env = mock_env("sender", &coins(0, "f"));
        let msg = HandleMsg::Register {
            snip_20: CanonicalAddr(Binary::from(b"address")),
        };
        let res = handle(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // TODO: Test fails, handle returns Err instead of Ok
        // TODO: Implement query to validate food_token?
    }
}
