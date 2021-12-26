use crate::msg::{FoodHandleMsg, HandleMsg, InitMsg};
use crate::state::{config, State};
use cosmwasm_std::{
    Api, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, Uint128,
};
use secret_toolkit::utils::HandleCallback;

impl HandleCallback for FoodHandleMsg {
    const BLOCK_SIZE: usize = 256;
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        food_token_code_hash: msg.food_token_code_hash.clone(),
        food_token_addr: msg.food_token_addr.clone(),
    };

    config(&mut deps.storage).save(&state)?;

    // FOOD token should be instantiated via cli.
    // This shoud set FOOD token address in state,
    // and also set Maket contract as FOOD token only minter.
    let food_init_msg = FoodHandleMsg::SetMinters {
        minters: vec![env.contract.address],
        padding: None,
    };

    let cosmos_msg =
        food_init_msg.to_cosmos_msg(msg.food_token_code_hash, msg.food_token_addr, None)?;

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
        HandleMsg::Buy {} => try_buy(deps, env),
    }
}

pub fn try_buy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    // TODO: Not sure why is sent_funds Vec,
    // skipping check if there are multiple funds of different denominations.
    let sent_funds = &env.message.sent_funds[0];
    if sent_funds.denom != "SCRT" {
        return Err(StdError::GenericErr {
            msg: "Invalid denomination".to_string(),
            backtrace: None,
        });
    }

    let state = config(&mut deps.storage).load()?;

    // Not sure what is the meaning of decimals in SNIP-20,
    // but this should enforce SCRT/FOOD ratio of 1:100.
    // This also means that decimals should be handled by UI.
    let amount = sent_funds.amount.u128() * 100;

    let food_mint_msg = FoodHandleMsg::Mint {
        recipient: env.message.sender,
        amount: Uint128::from(amount),
        padding: None,
    };

    let cosmos_msg =
        food_mint_msg.to_cosmos_msg(state.food_token_code_hash, state.food_token_addr, None)?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data: None,
    })
}
