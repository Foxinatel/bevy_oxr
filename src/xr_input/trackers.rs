use bevy::log::{debug, info};
use bevy::prelude::*;

use crate::{
    input::XrInput,
    resources::{XrFrameState, XrSession},
};

use super::{actions::XrActionSets, oculus_touch::OculusController, Hand, QuatConv, Vec3Conv};

#[derive(Component)]
pub struct OpenXRTrackingRoot;
#[derive(Component)]
pub struct OpenXRTracker;
#[derive(Component)]
pub struct OpenXRLeftEye;
#[derive(Component)]
pub struct OpenXRRightEye;
#[derive(Component)]
pub struct OpenXRHMD;
#[derive(Component)]
pub struct OpenXRLeftController;
#[derive(Component)]
pub struct OpenXRRightController;
#[derive(Component)]
pub struct OpenXRController;
#[derive(Component)]
pub struct AimPose(pub Transform);

pub fn adopt_open_xr_trackers(
    query: Query<Entity, (With<OpenXRTracker>, Without<Parent>)>,
    mut commands: Commands,
    tracking_root_query: Query<Entity, With<OpenXRTrackingRoot>>,
) {
    let root = tracking_root_query.get_single();
    if let Ok(root) = root {
        // info!("root is");
        for tracker in query.iter() {
            info!("we got a new tracker");
            commands.entity(root).add_child(tracker);
        }
    } else {
        info!("root isnt spawned yet?")
    }
}

pub fn verify_quat(mut quat: Quat) -> Quat {
    if quat.length() == 0.0 || !quat.is_finite() {
        quat = Quat::IDENTITY;
    }
    quat.normalize()
}

pub fn update_open_xr_controllers(
    oculus_controller: Res<OculusController>,
    mut left_controller_query: Query<
        (&mut Transform, Option<&mut AimPose>),
        (With<OpenXRLeftController>, Without<OpenXRRightController>),
    >,
    mut right_controller_query: Query<
        (&mut Transform, Option<&mut AimPose>),
        (With<OpenXRRightController>, Without<OpenXRLeftController>),
    >,
    frame_state: Res<XrFrameState>,
    xr_input: Res<XrInput>,
    session: Res<XrSession>,
    action_sets: Res<XrActionSets>,
) {
    //get controller
    let controller = oculus_controller.get_ref(&session, &frame_state, &xr_input, &action_sets);

    //set left controller position
    let (left_grip_space, _) = controller.grip_space(Hand::Left);
    let (left_aim_space, _) = controller.aim_space(Hand::Left);

    if let Ok((mut transform, pose)) = left_controller_query.get_single_mut() {
        if let Some(mut pose) = pose {
            *pose = AimPose(Transform {
                translation: left_aim_space.pose.position.to_vec3(),
                rotation: verify_quat(left_aim_space.pose.orientation.to_quat()),
                scale: Vec3::ONE,
            });
        }
        transform.translation = left_grip_space.pose.position.to_vec3();
        transform.rotation = verify_quat(left_grip_space.pose.orientation.to_quat());
    } else {
        debug!("no left controller entity found");
    }

    //set right controller position
    let (right_grip_space, _) = controller.grip_space(Hand::Right);
    let (right_aim_space, _) = controller.aim_space(Hand::Right);

    if let Ok((mut transform, pose)) = right_controller_query.get_single_mut() {
        if let Some(mut pose) = pose {
            *pose = AimPose(Transform {
                translation: right_aim_space.pose.position.to_vec3(),
                rotation: verify_quat(right_aim_space.pose.orientation.to_quat()),
                scale: Vec3::ONE,
            });
        }
        transform.translation = right_grip_space.pose.position.to_vec3();
        transform.rotation = verify_quat(right_grip_space.pose.orientation.to_quat());
    } else {
        debug!("no right controller entity found");
    }
}
