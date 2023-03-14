#[cfg(not(feature = "library"))]
pub mod contract {
  // version info for migration info
  use super::*;
  const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
  const CONTRACT_NAME: &str = "crates.io:cw-template";
  #[cfg(not(feature = "library"))]
  use cosmwasm_std::entry_point;
  use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult };
  use cw2::set_contract_version;

  use crate::error::ContractError;
  use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg };
  use crate::state::{ State, STATE };

  #[entry_point]
  pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
  ) -> Result<Response, ContractError> {
    let state = State {
      count: msg.count,
      owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(
      Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string())
    )
  }
  #[entry_point]
  pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
  ) -> Result<Response, ContractError> {
    match msg {
      ExecuteMsg::Increment {} => execute::increment(deps),
      ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    }
  }
  #[entry_point]
  pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
      QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    }
  }
}

pub mod execute {
  use cosmwasm_std::{ DepsMut, MessageInfo, Response };

  use crate::error::ContractError;
  use crate::state::{ STATE };
  pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(
      deps.storage,
      |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
      }
    )?;

    Ok(Response::new().add_attribute("action", "increment"))
  }

  pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(
      deps.storage,
      |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
          return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
      }
    )?;
    Ok(Response::new().add_attribute("action", "reset"))
  }
}

pub mod query {
  use cosmwasm_std::{ Deps, StdResult };

  use crate::msg::{ GetCountResponse };
  use crate::state::{ STATE };
  pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
  }
}
