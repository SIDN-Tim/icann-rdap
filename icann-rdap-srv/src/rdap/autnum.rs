use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::{error::RdapServerError, server::DynStoreState};

/// Gets an autnum object by the number path.
#[axum_macros::debug_handler]
#[tracing::instrument(level = "debug")]
pub(crate) async fn autnum_by_num(
    Path(as_num): Path<u32>,
    state: State<DynStoreState>,
) -> Result<Response, RdapServerError> {
    let storage = state.get_storage().await?;
    let autnum = storage.get_autnum_by_num(as_num).await?;
    Ok(autnum.response())
}