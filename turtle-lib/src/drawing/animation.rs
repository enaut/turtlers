mod circle_lens;
mod line_lens;

use bevy::prelude::{Quat, Transform, Vec2, Vec3};
use bevy_prototype_lyon::prelude::Path;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotateZLens},
    Animator, EaseFunction, Tween,
};

use crate::{
    general::{angle::Angle, length::Length, Coordinate, Precision},
    state::TurtleState,
};

use self::{
    circle_lens::{CircleAnimationLens, CircleMovementLens},
    line_lens::LineAnimationLens,
};

use super::{TurtleDrawCircle, TurtleDrawLine, TurtleGraphElement};

pub struct TurtleAnimationSegment {
    pub turtle_animation: Option<Tween<Transform>>,
    pub line_segment: Option<TurtleGraphElement>,
    pub line_animation: Option<Animator<Path>>,
}

pub trait ToAnimationSegment {
    fn to_draw_segment(
        &self,
        state: &mut TurtleState,
    ) -> crate::drawing::animation::TurtleAnimationSegment;
}

pub fn turtle_turn(
    state: &mut TurtleState,
    angle_to_turn: Angle<Precision>,
) -> TurtleAnimationSegment {
    let start = state.heading();
    let end = state.heading() + angle_to_turn;
    let animation = Tween::new(
        EaseFunction::QuadraticInOut,
        state.animation_duration(),
        TransformRotateZLens {
            start: start.to_radians().value(),
            end: end.to_radians().value(),
        },
    )
    .with_completed_event(state.segment_index() as u64);
    // Don't draw as the position does not change
    let line = TurtleGraphElement::Noop;
    // Update the state
    state.set_heading(end.limit_smaller_than_full_circle());
    TurtleAnimationSegment {
        turtle_animation: Some(animation),
        line_segment: Some(line),
        line_animation: None,
    }
}

pub fn move_straight_segment(state: &mut TurtleState, length: Precision) -> TurtleAnimationSegment {
    let animation = MoveStraightTurtleAnimation::new(state, length);

    state.set_position(animation.end);
    TurtleAnimationSegment {
        turtle_animation: Some(animation.animation),
        line_segment: None,
        line_animation: None,
    }
}

pub fn draw_straight_segment(state: &mut TurtleState, length: Precision) -> TurtleAnimationSegment {
    let animation = MoveStraightTurtleAnimation::new(state, length);
    let line_animation = MoveStraightLineAnimation::new(state, length, &animation);

    state.set_position(animation.end);
    TurtleAnimationSegment {
        turtle_animation: Some(animation.animation),
        line_segment: Some(TurtleGraphElement::TurtleLine(line_animation.line)),
        line_animation: Some(Animator::new(line_animation.animation)),
    }
}

pub fn draw_circle_segment(
    state: &mut TurtleState,
    radius: Length,
    angle: Angle<Precision>,
) -> TurtleAnimationSegment {
    let animation = MoveCircleTurtleAnimation::new(state, radius, angle);
    let line_animation = MoveCircleLineAnimation::new(state, radius, angle);
    state.set_position(animation.end);
    state.set_heading(animation.end_heading);
    TurtleAnimationSegment {
        turtle_animation: Some(animation.animation),
        line_segment: Some(TurtleGraphElement::TurtleCircle(line_animation.line)),
        line_animation: Some(Animator::new(line_animation.animation)),
    }
}

struct MoveStraightLineAnimation {
    _start: Coordinate,
    _end: Coordinate,
    line: TurtleDrawLine,
    animation: Tween<Path>,
}

impl MoveStraightLineAnimation {
    fn new(
        state: &TurtleState,
        _length: Precision,
        turtle_animation: &MoveStraightTurtleAnimation,
    ) -> Self {
        let line = TurtleDrawLine::new(turtle_animation.start, turtle_animation.end);
        let line_animation = Tween::new(
            EaseFunction::QuadraticInOut,
            state.animation_duration(),
            LineAnimationLens::new(turtle_animation.start, turtle_animation.end),
        )
        /* .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
        .with_repeat_count(RepeatCount::Infinite)*/;
        Self {
            _start: turtle_animation.start,
            _end: turtle_animation.end,
            line,
            animation: line_animation,
        }
    }
}

struct MoveStraightTurtleAnimation {
    start: Coordinate,
    end: Coordinate,
    animation: Tween<Transform>,
}
impl MoveStraightTurtleAnimation {
    fn new(state: &TurtleState, length: Precision) -> Self {
        let start = state.position();
        let end =
            state.position() + (Vec2::from_angle(state.heading().to_radians().value()) * length);
        let turtle_movement_animation = Tween::new(
            EaseFunction::QuadraticInOut,
            state.animation_duration(),
            TransformPositionLens {
                start: start.extend(0.),
                end: end.extend(0.),
            },
        )
        .with_completed_event(state.segment_index() as u64);
        Self {
            start,
            end,
            animation: turtle_movement_animation,
        }
    }
}

struct MoveCircleLineAnimation {
    _start: Coordinate,
    _end: Coordinate,
    line: TurtleDrawCircle,
    animation: Tween<Path>,
}

impl MoveCircleLineAnimation {
    fn new(state: &TurtleState, radius: Length, angle: Angle<Precision>) -> Self {
        let radii = Vec2::ONE * radius.0.abs();
        let start = state.position();
        let left_right = Angle::degrees(if radius.0 >= 0. { 90. } else { -90. });
        let center = state.position()
            + (Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                ((state.heading() + left_right).to_radians()).value(),
            )));
        let end_pos = center
            + Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                (state.heading() + angle - left_right).to_radians().value(),
            ));

        let line =
            TurtleDrawCircle::new(center, radii, Angle::degrees(0.), state.position(), end_pos);
        let line_animator = Tween::new(
            EaseFunction::QuadraticInOut,
            state.animation_duration(),
            CircleAnimationLens {
                start_pos: state.position(),
                center,
                radii,
                start: Angle::degrees(0.),
                end: if radius.0 > 0. { angle } else { -angle },
            },
        );
        Self {
            _start: start,
            _end: end_pos,
            line,
            animation: line_animator,
        }
    }
}

struct MoveCircleTurtleAnimation {
    _start: Coordinate,
    end: Coordinate,
    end_heading: Angle<Precision>,
    animation: Tween<Transform>,
}

impl MoveCircleTurtleAnimation {
    fn new(state: &TurtleState, radius: Length, angle: Angle<Precision>) -> Self {
        let start = state.position();
        let left_right = Angle::degrees(if radius.0 >= 0. { 90. } else { -90. });
        let center = state.position()
            + (Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                ((state.heading() + left_right).to_radians()).value(),
            )));
        let end_heading = state.heading() + if radius.0 > 0. { angle } else { -angle };
        let end_pos = center
            + Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                (state.heading() + angle - left_right).to_radians().value(),
            ));
        let turtle_movement_animation = Tween::new(
            EaseFunction::QuadraticInOut,
            state.animation_duration(),
            CircleMovementLens {
                start: Transform {
                    translation: state.position().extend(0.),
                    rotation: Quat::from_rotation_z(state.heading().to_radians().value()),
                    scale: Vec3::ONE,
                },
                end: if radius.0 > 0. { angle } else { -angle },
                center,
            },
        )
        .with_completed_event(state.segment_index() as u64);
        Self {
            _start: start,
            end: end_pos,
            end_heading,
            animation: turtle_movement_animation,
        }
    }
}
