pub mod helper;
pub mod program_coord_guidance;
pub mod stationary_entity_guidance;
pub mod straight_guidance;
pub mod straight_guidance_wo_gravity;

use std::collections::HashMap;
use tokio_stream::{Stream, StreamExt};
use url::Url;

use anyhow::{Context, Result};

use crate::guidance_grpc::{ControlInput, MissileState};

pub trait MissileGuidance: Sized {
    fn new(
        params: HashMap<String, String>,
        initial_missile_state: &MissileState,
    ) -> impl std::future::Future<Output = Result<Self>>;

    // The id of the ControlInput will be set by the caller.
    fn get_guidance(
        &mut self,
        missile_state: MissileState,
    ) -> impl std::future::Future<Output = ControlInput>;
}

async fn parse_missile_name(
    initial_missile_state: &MissileState,
) -> Result<(String, HashMap<String, String>)> {
    let fake_base = Url::parse("https://chris-besch.com/")?;
    let fake_url = fake_base.join(
        &(initial_missile_state
            .missile
            // TODO: clone can probably be circumvented
            .clone()
            .context("initial MissileState doesn't have missile set")?
            .name),
    )?;
    Ok((
        fake_url.path().to_string(),
        fake_url.query_pairs().into_owned().collect(),
    ))
}

// We need to use the first MissileState to figure out what guidance code to use.
// Once we have constructed the MissileGuidance implementation we can use the generic function for
// that MissileGuidance implementation to handle all following MissileStates.
// That's why we need this stream inside a stream architecture:
// The outer stream receives the initial MissileState and then hands over to the other stream
// defined in the generic function get_guidance_stream_after_launch.
pub fn get_guidance_stream(
    mut missile_state_stream: tonic::Streaming<MissileState>,
) -> impl Stream<Item = std::result::Result<ControlInput, tonic::Status>> {
    async_stream::try_stream! {
        // get the first MissileState
        let initial_missile_state = missile_state_stream.next().await;
        if let Some(Ok(initial_missile_state)) = initial_missile_state {
            // Figure out what guidance to use
            match parse_missile_name(&initial_missile_state).await {
                Ok((guidance_type, params)) => {
                    match (guidance_type.as_str(), params) {
                        ("/straightWOGravity", params) => {
                            // We know what guidance to use, hand the rest over.
                            let mut guidance_stream = get_guidance_stream_after_launch(
                                straight_guidance_wo_gravity::StraightGuidanceWOGravity::new(
                                    params,
                                    &initial_missile_state,
                                )
                                .await,
                                initial_missile_state,
                                missile_state_stream,
                            );
                            while let Some(control_input) = guidance_stream.next().await {
                                yield control_input;
                            }
                        }
                        ("/straight", params) => {
                            // We know what guidance to use, hand the rest over.
                            let mut guidance_stream = get_guidance_stream_after_launch(
                                straight_guidance::StraightGuidance::new(
                                    params,
                                    &initial_missile_state,
                                )
                                .await,
                                initial_missile_state,
                                missile_state_stream,
                            );
                            while let Some(control_input) = guidance_stream.next().await {
                                yield control_input;
                            }
                        }
                        ("/coord", params) => {
                            // We know what guidance to use, hand the rest over.
                            let mut guidance_stream = get_guidance_stream_after_launch(
                                program_coord_guidance::TargetCoordGuidance::new(
                                    params,
                                    &initial_missile_state,
                                )
                                .await,
                                initial_missile_state,
                                missile_state_stream,
                            );
                            while let Some(control_input) = guidance_stream.next().await {
                                yield control_input;
                            }
                        }
                        ("/entity", params) => {
                            // We know what guidance to use, hand the rest over.
                            let mut guidance_stream = get_guidance_stream_after_launch(
                                stationary_entity_guidance::StationaryEntityGuidance::new(
                                    params,
                                    &initial_missile_state,
                                )
                                .await,
                                initial_missile_state,
                                missile_state_stream,
                            );
                            while let Some(control_input) = guidance_stream.next().await {
                                yield control_input;
                            }
                        }
                        (guidance_type, _) => {
                            eprintln!("unknown guidance type: {}", guidance_type);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("failed to parse missile name: {:?}", e);
                }
            }
        } else {
            eprintln!(
                "failed to receive an initial MissileState: {:?}",
                initial_missile_state
            );
        }
    }
}

fn get_guidance_stream_after_launch<G: MissileGuidance>(
    guidance_res: Result<G>,
    initial_missile_state: MissileState,
    mut missile_state_stream: tonic::Streaming<MissileState>,
) -> impl Stream<Item = ControlInput> {
    Box::pin(async_stream::stream! {
        let mut id: i32 = 0;
        match guidance_res {
            Ok(mut guidance) => {
                // handle the initial MissileState, which was used to determine what guidance code to use
                {
                    let mut control_input = guidance.get_guidance(initial_missile_state).await;
                    id += 1;
                    control_input.id = id;
                    println!("{:?}", control_input);
                    yield control_input;
                }

                while let Some(missile_state_res) = missile_state_stream.next().await {
                    match missile_state_res {
                        Ok(missile_state) => {
                            if missile_state.destroyed {
                                println!("received destroyed missile state, good bye");
                                break;
                            }
                            let mut control_input = guidance.get_guidance(missile_state).await;
                            id += 1;
                            control_input.id = id;
                            println!("{:?}", control_input);
                            yield control_input;
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("discarding missile at launch: {:?}", e);
                id += 1;
                yield ControlInput {
                    id: (id),
                    hardware_config: None,
                    pitch_turn: 0.0,
                    yaw_turn: 0.0,
                    explode: false,
                    disarm: true,
                };
            }
        }
    })
}
