use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, LogAttribute,
    StdError, StdResult, Storage,HumanAddr,
};
use schemars::{JsonSchema,schema_for};
use std::collections::HashMap;
use crate::msg::{QueryScoreResponse,QueryMemberResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{load, may_load, save,config, config_read, State,CONFIG_KEY};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut addr_member_: HashMap<String, String> = HashMap::new();
    let mut member_scoring_: HashMap<String,HashMap<String,i32>> = HashMap::new();
    if msg.addr.len() != msg.member.len() {
        return Err(StdError::GenericErr { 
            msg: "addr member length unequal".to_string(),
            backtrace: None
        });
    }
    for i in 0..msg.addr.len(){
        addr_member_.insert(msg.addr[i].clone(),msg.member[i].clone());
    }

    for member in msg.member{
        let mut scoring: HashMap<String,i32> = HashMap::new();
        member_scoring_.insert(member.to_string(),scoring);
    }

    let state = State {
        group_name: msg.group_name,
        //addr_member: msg.addr_member,
        addr_member: addr_member_,
        member_scoring: member_scoring_,
        //leader: env.message.sender.clone(),
    };

    //config(&mut deps.storage).save(&state)?;
    /*for (member,scoring) in &state.member_scoring{
        println!("member {}'s scoring:",member);
        for (scoree,score) in scoring{
            println!("{}:{}",scoree,score);
        }
    }*/
    

    save(&mut deps.storage, CONFIG_KEY, &state)?;
    let mut la = LogAttribute::default();
    la.value = "test".to_string();
    
    //Ok(InitResponse::default());
    Ok(InitResponse {
        messages: vec![],
        log: vec![la],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Score{member,point} => try_score(deps, env,member,point),
    }
}

pub fn try_score<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    member: String,
    point: i32,
) -> StdResult<HandleResponse> {
    let mut state: State = load(& deps.storage, CONFIG_KEY)?;


    for (member,scoring) in &state.member_scoring{
        println!("member {}'s scoring:",member);
        for (scoree,score) in scoring{
            println!("{}:{}",scoree,score);
        }
    } 

    //let state: State = config_read(&deps.storage).load()?;
    let sender_addr = env.message.sender;
    if !state.addr_member.contains_key(&sender_addr.as_str().to_string()){
        return Err(StdError::GenericErr { 
            msg: "not in group".to_string(),
            backtrace: None
        });
    }

    if !score_valid(point){
        return Err(StdError::GenericErr { 
            msg: "Score invalid".to_string(),
            backtrace: None
        });
    }
    /*config(&mut deps.storage).update(|mut state| {
        if let Some(scoring) = state.member_scoring.get_mut(&state.addr_member[&sender_addr.as_str().to_string()]) {
            if let Some(score) = scoring.get_mut(&member) {
                *score = point;
            }
        }
        Ok(state)
    })?;*/
    println!("sender_addr:{}",sender_addr);
    if let Some(scoring) = state.member_scoring.get_mut(&state.addr_member[&sender_addr.as_str().to_string()]) {
        if let Some(score) = scoring.get_mut(&member) {
            *score = point;
            println!("scored");
        }else{
            scoring.insert(member,point);
            println!("1st scored");
        }
    }

    save(&mut deps.storage, CONFIG_KEY, &state)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&"scored successfully".to_string())?),
    })
}

fn score_valid(score:i32) -> bool{
    return score >=0 && score <= 100;
}


pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMemberScore{member} => to_binary(&query_score(deps,member)?),
        QueryMsg::GetMember{} => to_binary(&query_member(deps)?),
    }
}

fn query_score<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    queried_member : String,
) -> StdResult<QueryScoreResponse> {
    let mut avgscore = 0;
    let mut member_count = 0;
    let mut state: State = load(& deps.storage, CONFIG_KEY)?;
    //let state = config_read(&deps.storage).load()?;
    for (_member,scoring) in &state.member_scoring{
        if !scoring.contains_key(&queried_member){
            return Err(StdError::GenericErr { 
                msg: "Not all members scored".to_string(),
                backtrace: None
            });
        }
        avgscore += scoring[&queried_member];
        member_count += 1;
    } 
    avgscore /= member_count;
    Ok(QueryScoreResponse { 
        member_score: avgscore 
    })
}

fn query_member<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<QueryMemberResponse> {
    let state: State = load(& deps.storage, CONFIG_KEY)?;
    //let state = load(&deps.storage).load()?;
    Ok(QueryMemberResponse { 
        group_name : state.group_name,
        members : state.addr_member.values().cloned().collect::<Vec<String>>(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let schema = schema_for!(InitMsg);
        println!("\n\n\n{}\n\n\n", serde_json::to_string_pretty(&schema).unwrap());
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { 
            group_name: "testGroup".to_string(), 
            addr: vec!["mockaddr1".to_string(),"mockaddr2".to_string(),"mockaddr3".to_string(),"mockaddr4".to_string()],
            member: vec!["alice".to_string(),"bryan".to_string(),"cindy".to_string(),"dylan".to_string()], 
        };

        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetMember {}).unwrap();
        let value: QueryMemberResponse = from_binary(&res).unwrap();
        for m in value.members{
            println!("member {}",m);
        }
        assert_eq!("testGroup".to_string(), value.group_name);
    }
    #[test]
    fn always_true() {
        assert_eq!(1,1);
    }

    #[test]
    fn score() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { 
            group_name: "testGroup".to_string(), 
            addr: vec!["mockaddr1".to_string(),"mockaddr2".to_string(),"mockaddr3".to_string(),"mockaddr4".to_string()],
            member: vec!["alice".to_string(),"bryan".to_string(),"cindy".to_string(),"dylan".to_string()], 
        };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // score
        let env = mock_env("mockaddr1", &coins(2, "token"));
        let msg = HandleMsg::Score {member:"alice".to_string(),point:10};
        let _res = handle(&mut deps, env, msg);
        
        let env = mock_env("mockaddr2", &coins(2, "token"));
        let msg = HandleMsg::Score {member:"alice".to_string(),point:20};
        let _res = handle(&mut deps, env, msg);
        
        let env = mock_env("mockaddr3", &coins(2, "token"));
        let msg = HandleMsg::Score {member:"alice".to_string(),point:30};
        let _res = handle(&mut deps, env, msg);
        
        let env = mock_env("mockaddr4", &coins(2, "token"));
        let msg = HandleMsg::Score {member:"alice".to_string(),point:40};
        let _res = handle(&mut deps, env, msg);

        let res = query(&deps, QueryMsg::GetMemberScore {member:"alice".to_string()}).unwrap();
        let value: QueryScoreResponse = from_binary(&res).unwrap();
        assert_eq!(25, value.member_score);

        // rescore
        let env = mock_env("mockaddr1", &coins(2, "token"));
        let msg = HandleMsg::Score {member:"alice".to_string(),point:30};
        let _res = handle(&mut deps, env, msg);

        let res = query(&deps, QueryMsg::GetMemberScore {member:"alice".to_string()}).unwrap();
        let value: QueryScoreResponse = from_binary(&res).unwrap();
        assert_eq!(30, value.member_score);

    }
}
