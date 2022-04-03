use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{InfoResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        course_name: msg.course_name,
        course_ticket: msg.course_ticket,
        price: msg.price,
        sell_end: false,
        owner: deps.api.canonical_address(&env.message.sender)?,
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
        HandleMsg::BuyCourseTicket {amount} => try_buy(deps, env,amount),
        HandleMsg::CancelSell {} => try_cancel(deps, env),
        HandleMsg::Reprice { price } => try_reprice(deps, env, price),
    }
}

pub fn try_buy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    amount: i32,
) -> StdResult<HandleResponse> {
    let state: State = config_read(&deps.storage).load()?;
    if state.sell_end{
        return Err(StdError::GenericErr { 
            msg: "Selling has ended".to_string(),
            backtrace: None
        });
    }
    if amount < state.price{
        return Err(StdError::GenericErr { 
            msg: "Not enough".to_string(),
            backtrace: None
        });
    }

    config(&mut deps.storage).update(|mut state| {
        state.sell_end = true;
        Ok(state)
    })?;

    debug_print("bought successfully");
    let mut res = HandleResponse::default();
    res.data = Some(Binary::from_base64(&*state.course_ticket)?);
    Ok(res)
}

pub fn try_cancel<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    config(&mut deps.storage).update(|mut state| {
        if sender_address_raw != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        state.sell_end = true;
        Ok(state)
    })?;

    debug_print("sell ended successfully");
    Ok(HandleResponse::default())
}


pub fn try_reprice<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    price: i32,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    config(&mut deps.storage).update(|mut state| {
        if sender_address_raw != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        state.price = price;
        Ok(state)
    })?;
    debug_print("repriced successfully");
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetInfo {} => to_binary(&query_info(deps)?),
    }
}

fn query_info<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<InfoResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(InfoResponse { 
        course_name: state.course_name,
        price: state.price,
        sell_end: state.sell_end, 
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { price: 10,course_name:"forest".to_string(),course_ticket:"7777".to_string()};
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetInfo {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!("forest".to_string(), value.course_name);
        assert_eq!(10, value.price);
        assert_eq!(false, value.sell_end);
    }

    #[test]
    fn buy() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {price: 10,course_name:"forest".to_string(),course_ticket:"7777".to_string()};
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can buy
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::BuyCourseTicket {amount:10};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should end
        let res = query(&deps, QueryMsg::GetInfo {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.sell_end);
    }

    #[test]
    fn buyfail() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {price: 10,course_name:"forest".to_string(),course_ticket:"7777".to_string()};
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // not enough money
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::BuyCourseTicket {amount:5};
        let _res = handle(&mut deps, env, msg);
        match _res {
            Err(StdError::GenericErr { .. }) => {}
            _ => panic!("Must return generic error"),
        }
        // should not end
        let res = query(&deps, QueryMsg::GetInfo {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!(false, value.sell_end);


        // anyone can buy
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::BuyCourseTicket {amount:10};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should end
        let res = query(&deps, QueryMsg::GetInfo {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.sell_end);
    }
}
