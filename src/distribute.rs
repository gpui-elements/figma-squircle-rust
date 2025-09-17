use std::collections::HashMap;
use phf::phf_map;

use crate::{collection};

pub struct RoundedRectangle {
    pub top_left_corner_radius: f32,
    pub top_right_corner_radius: f32,
    pub bottom_right_corner_radius: f32,
    pub bottom_left_corner_radius: f32,
    pub width: f32,
    pub height: f32
}

pub struct NormalizedCorner {
    pub radius: f32,
    pub rounding_and_smoothing_budget: f32
}

pub struct NormalizedCorners {
    pub top_left: NormalizedCorner,
    pub top_right: NormalizedCorner,
    pub bottom_left: NormalizedCorner,
    pub bottom_right: NormalizedCorner
}

pub fn distribute_and_normalize(RoundedRectangle {
    top_left_corner_radius,
    top_right_corner_radius,
    bottom_right_corner_radius,
    bottom_left_corner_radius,
    width,
    height,
}: RoundedRectangle) -> NormalizedCorners {
    let mut rounding_and_smoothing_budget_map: HashMap<Corner, f32> = collection! {
        Corner::TopLeft => -1.0,
        Corner::TopRight => -1.0,
        Corner::BottomLeft => -1.0,
        Corner::BottomRight => -1.0,
    };

    let mut corner_radius_map: HashMap<Corner, f32> = collection! {
        Corner::TopLeft => top_left_corner_radius,
        Corner::TopRight => top_right_corner_radius,
        Corner::BottomLeft => bottom_left_corner_radius,
        Corner::BottomRight => bottom_right_corner_radius
    };

    let mut corner_radius_map_entries = [
        (Corner::TopLeft, top_left_corner_radius),
        (Corner::TopRight, top_right_corner_radius),
        (Corner::BottomLeft, bottom_left_corner_radius),
        (Corner::BottomRight, bottom_right_corner_radius)
    ];
    corner_radius_map_entries.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    for (corner, radius) in corner_radius_map_entries {
        let Some(adjacents) = ADJACENTS_BY_CORNER.get(&(corner as usize)) else { continue };

        // Look at the 2 adjacent sides, figure out how much space we can have on both sides,
        // then take the smaller one
        let budget = adjacents.iter()
            .filter_map(|(adjaccent_corner, adjacent_side)| {
                let Some(adjacent_corner_radius) = corner_radius_map.get(adjaccent_corner) else { return None };

                if radius == 0.0 && adjacent_corner_radius == &0.0 {
                    return Some(0.0)
                }

                let Some(adjacent_corner_budget) = rounding_and_smoothing_budget_map.get(adjaccent_corner) else { return None };

                let side_length = if matches!(adjacent_side, Side::Top | Side::Bottom) { width } else { height };

                // If the adjacent corner's already been given the rounding and smoothing budget,
                // we'll just take the rest
                if adjacent_corner_budget >= &0.0 {
                    return Some(side_length - adjacent_corner_budget)
                } else {
                    return Some((radius / (radius + adjacent_corner_radius)) * side_length)
                }
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_default();

        rounding_and_smoothing_budget_map.insert(corner.clone(), budget);
        corner_radius_map.insert(corner, radius.min(budget));
    }

    return NormalizedCorners {
        top_left: NormalizedCorner {
            radius: *corner_radius_map.get(&Corner::TopLeft).unwrap_or(&0.0),
            rounding_and_smoothing_budget: *rounding_and_smoothing_budget_map.get(&Corner::TopLeft).unwrap_or(&0.0)
        },
        top_right: NormalizedCorner {
            radius: *corner_radius_map.get(&Corner::TopRight).unwrap_or(&0.0),
            rounding_and_smoothing_budget: *rounding_and_smoothing_budget_map.get(&Corner::TopRight).unwrap_or(&0.0)
        },
        bottom_left: NormalizedCorner {
            radius: *corner_radius_map.get(&Corner::BottomLeft).unwrap_or(&0.0),
            rounding_and_smoothing_budget: *rounding_and_smoothing_budget_map.get(&Corner::BottomLeft).unwrap_or(&0.0)
        },
        bottom_right: NormalizedCorner {
            radius: *corner_radius_map.get(&Corner::BottomRight).unwrap_or(&0.0),
            rounding_and_smoothing_budget: *rounding_and_smoothing_budget_map.get(&Corner::BottomRight).unwrap_or(&0.0)
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Corner {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3
}

pub enum Side {
    Top = 0,
    Left = 1,
    Bottom = 2,
    Right = 3
}

const ADJACENTS_BY_CORNER: phf::Map<usize, &'static [(Corner, Side)]> = phf_map! {
    // TopLeft
    0usize => &[
        (Corner::TopRight, Side::Top),
        (Corner::BottomLeft, Side::Left)
    ],

    // TopRight
    1usize => &[
        (Corner::TopLeft, Side::Top),
        (Corner::BottomRight, Side::Right)
    ],

    // BottomLeft
    2usize => &[
        (Corner::BottomRight, Side::Bottom),
        (Corner::TopLeft, Side::Left)
    ],

    // BottomRight
    3usize => &[
        (Corner::BottomLeft, Side::Bottom),
        (Corner::TopRight, Side::Right)
    ]
};