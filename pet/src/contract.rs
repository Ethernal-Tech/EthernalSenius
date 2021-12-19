use crate::msg::{FoodBurnFromMsg, HandleMsg, InitMsg, QueryMsg, QueryResponse};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};
use secret_toolkit::utils::InitCallback;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

impl InitCallback for FoodBurnFromMsg {
    const BLOCK_SIZE: usize = 256;
}

const CODE_ID: u64 = 1;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let state = State {
        starved: false,
        last_meal: since_epoch.as_millis(),
        owner: env.message.sender.clone(),
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Feed { meals } => try_feed(deps, env, meals),
    }
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    meals: u128,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        let four_hours = Duration::from_secs(4 * 60 * 60).as_millis();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let four_hours_ago = now - four_hours;

        if env.message.sender != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        if state.starved {
            return Err(StdError::GenericErr {
                msg: "dead".to_string(),
                backtrace: None,
            });
        }
        if meals == 0 {
            return Err(StdError::GenericErr {
                msg: "no meals".to_string(),
                backtrace: None,
            });
        }
        if state.last_meal < four_hours_ago {
            state.starved = true;
            return Err(StdError::GenericErr {
                msg: "died".to_string(),
                backtrace: None,
            });
        }
        Ok(state)
    })?;

    let burn_from_msg = FoodBurnFromMsg {
        owner: env.message.sender,
        amount: Uint128(meals),
        padding: None,
    };
    let cosmos_msg =
        burn_from_msg.to_cosmos_msg("LABEL".to_string(), CODE_ID, "CODE_HASH".to_string(), None)?;

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
    let _state = config_read(&deps.storage).load()?;
    Ok(QueryResponse { is_alive: true })
}
