pub mod straight_guidance;
pub mod straight_guidance_wo_gravity;

use tokio_stream::{Stream, StreamExt};

use crate::{
    guidance_grpc::{guidance_server::Guidance, ControlInput, MissileState},
    MyGuidance,
};

pub trait MissileGuidance {
    // The id of the ControlInput will be set by the caller.
    fn get_guidance(
        &mut self,
        missile_state: MissileState,
    ) -> impl std::future::Future<Output = ControlInput>;
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
        let initial_missile_state = missile_state_stream.next().await;
        if let Some(Ok(initial_missile_state)) = initial_missile_state {
            // TODO: get actual guidance
            let guidance = straight_guidance::StraightMissileGuidance {
                target_pitch: 0.0,
                target_yaw: 0.0,
                hardware_config: None,
            };

            // hand the rest over
            let mut guidance_stream = get_guidance_stream_after_launch(guidance, initial_missile_state, missile_state_stream);
            while let Some(control_input) = guidance_stream.next().await {
                yield control_input;
            }
        }
        else {
            eprintln!("failed to receive an initial MissileState: {:?}", initial_missile_state);
        }
    }
}

fn get_guidance_stream_after_launch<G: MissileGuidance>(
    mut guidance: G,
    initial_missile_state: MissileState,
    mut missile_state_stream: tonic::Streaming<MissileState>,
) -> impl Stream<Item = ControlInput> {
    Box::pin(async_stream::stream! {
        let mut id: i32 = 0;

        // handle the initial MissileState, which was used to determine what guidance code to use
        {
            let mut control_input = guidance.get_guidance(initial_missile_state).await;
            id += 1;
            control_input.id = id;
            yield control_input;
        }

        while let Some(missile_state_res) = missile_state_stream.next().await {
            match missile_state_res{
                Ok(missile_state) => {
                    let mut control_input = guidance.get_guidance(missile_state).await;
                    id += 1;
                    control_input.id = id;
                    yield control_input;
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                    break;
                }
            }
        }
    })
}
