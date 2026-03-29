use axum::{extract::State, Json};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    args::Args,
    controller::{
        error::{Error, Result},
        response::{AllocateRequest, AllocateResponse},
    },
    k8s::{allocation::create_allocation, K8sClient},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Args>,
    pub k8s_client: K8sClient,
}

pub async fn allocate(
    State(state): State<AppState>,
    Json(AllocateRequest{ schema }): Json<AllocateRequest>,
) -> Result<Json<AllocateResponse>> {
    // Create allocation
    let response = create_allocation(
        state.k8s_client.client(),
        &state.config.namespace,
        &state.config.fleet_name,
        state.config.scheduling.as_str(),
        schema,
    ).await?;

    Ok(Json(response))
}
