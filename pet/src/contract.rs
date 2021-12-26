use crate::msg::{FoodHandleMsg, HandleMsg, InitMsg, QueryMsg, QueryResponse};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};
use secret_toolkit::utils::HandleCallback;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const FOUR_HOURS: u128 = Duration::from_secs(4 * 60 * 60).as_millis();

impl HandleCallback for FoodHandleMsg {
    const BLOCK_SIZE: usize = 256;
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let state = State {
        starved: false,
        food_token_code_hash: msg.food_token_code_hash,
        food_token_addr: msg.food_token_addr,
        full_until: since_epoch.as_millis() + FOUR_HOURS,
        owner: env.message.sender.clone(),
    };

    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Feed {} => try_feed(deps, env),
    }
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    if env.message.sent_funds.len() != 1 {
        return Err(StdError::GenericErr {
            msg: "No funds or multiple sent funds".to_string(),
            backtrace: None,
        });
    }
    let sent_funds = &env.message.sent_funds[0];
    if sent_funds.denom != "FOOD" {
        return Err(StdError::GenericErr {
            msg: "Invalid denomination".to_string(),
            backtrace: None,
        });
    }
    if sent_funds.amount.u128() < 1 {
        return Err(StdError::GenericErr {
            msg: "Invalid amount (<0)".to_string(),
            backtrace: None,
        });
    }
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let state = config(&mut deps.storage).load()?;
    if state.starved {
        return Err(StdError::GenericErr {
            msg: "dead".to_string(),
            backtrace: None,
        });
    }

    let mut starved = false;
    config(&mut deps.storage).update(|mut state| {
        if env.message.sender != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }

        if state.full_until < since_epoch {
            state.starved = true;
            starved = true;
        } else {
            state.full_until = since_epoch + FOUR_HOURS;
        }
        Ok(state)
    })?;

    if starved {
        return Err(StdError::GenericErr {
            msg: "died".to_string(),
            backtrace: None,
        });
    }

    let burn_from_msg = FoodHandleMsg::BurnFrom {
        owner: env.message.sender.clone(),
        amount: sent_funds.amount,
        padding: None,
    };

    let cosmos_msg =
        burn_from_msg.to_cosmos_msg(state.food_token_code_hash, state.food_token_addr, None)?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Query {} => to_binary(&query_all(deps)?),
    }
}

fn query_all<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    Ok(QueryResponse {
        is_alive: !state.starved && state.full_until > now,
    })
}
