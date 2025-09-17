mod distribute;
use distribute::{NormalizedCorners, RoundedRectangle, distribute_and_normalize};

mod draw;
use draw::{CornerParams, SvgPathInput, get_path_params_for_corner, get_svg_path_from_path_params};

#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

#[derive(Debug)]
pub struct FigmaSquircleParams {
    pub width: f32,
    pub height: f32,
    pub corner_radius: Option<f32>,
    pub top_left_corner_radius: Option<f32>,
    pub top_right_corner_radius: Option<f32>,
    pub bottom_right_corner_radius: Option<f32>,
    pub bottom_left_corner_radius: Option<f32>,
    pub corner_smoothing: f32,
    pub preserve_smoothing: bool,
}

impl Default for FigmaSquircleParams {
    fn default() -> Self {
        Self {
            width: 0.,
            height: 0.,
            corner_radius: None,
            top_left_corner_radius: None,
            top_right_corner_radius: None,
            bottom_right_corner_radius: None,
            bottom_left_corner_radius: None,
            corner_smoothing: 1.,
            preserve_smoothing: false
        }
    }
}

impl FigmaSquircleParams {
    pub fn width(self, width: f32) -> Self {
        Self {
            width,
            ..self
        }
    }

    pub fn height(self, height: f32) -> Self {
        Self {
            height,
            ..self
        }
    }

    pub fn size(self, size: f32) -> Self {
        Self {
            width: size,
            height: size,
            ..self
        }
    }

    pub fn corner_radius(self, corner_radius: f32) -> Self {
        Self {
            corner_radius: Some(corner_radius),
            ..self
        }
    }

    pub fn top_left_corner_radius(self, corner_radius: f32) -> Self {
        Self {
            top_left_corner_radius: Some(corner_radius),
            ..self
        }
    }

    pub fn top_right_corner_radius(self, corner_radius: f32) -> Self {
        Self {
            top_right_corner_radius: Some(corner_radius),
            ..self
        }
    }

    pub fn bottom_left_corner_radius(self, corner_radius: f32) -> Self {
        Self {
            bottom_left_corner_radius: Some(corner_radius),
            ..self
        }
    }

    pub fn bottom_right_corner_radius(self, corner_radius: f32) -> Self {
        Self {
            bottom_right_corner_radius: Some(corner_radius),
            ..self
        }
    }

    pub fn corner_smoothing(self, corner_smoothing: f32) -> Self {
        Self {
            corner_smoothing,
            ..self
        }
    }

    pub fn preserve_smoothing(self, preserve_smoothing: bool) -> Self {
        Self {
            preserve_smoothing,
            ..self
        }
    }
}

pub fn get_svg_path(FigmaSquircleParams {
    corner_radius,
    top_left_corner_radius,
    top_right_corner_radius,
    bottom_right_corner_radius,
    bottom_left_corner_radius,
    corner_smoothing,
    width,
    height,
    preserve_smoothing
}: FigmaSquircleParams) -> String {
    let corner_radius = corner_radius.unwrap_or(0.0);
    let top_left_corner_radius = top_left_corner_radius.unwrap_or(corner_radius);
    let top_right_corner_radius = top_right_corner_radius.unwrap_or(corner_radius);
    let bottom_right_corner_radius = bottom_right_corner_radius.unwrap_or(corner_radius);
    let bottom_left_corner_radius = bottom_left_corner_radius.unwrap_or(corner_radius);

    if top_left_corner_radius == top_right_corner_radius &&
        top_right_corner_radius == bottom_right_corner_radius &&
        bottom_right_corner_radius == bottom_left_corner_radius &&
        bottom_left_corner_radius == top_left_corner_radius
    {
        let rounding_and_smoothing_budget = width.min(height) / 2.0;
        let corner_radius = top_left_corner_radius.min(rounding_and_smoothing_budget);

        let path_params = get_path_params_for_corner(CornerParams {
            corner_radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget
        });

        return get_svg_path_from_path_params(SvgPathInput {
            width,
            height,
            top_right_path_params: &path_params,
            bottom_right_path_params: &path_params,
            bottom_left_path_params: &path_params,
            top_left_path_params: &path_params
        })
    }

    let NormalizedCorners {
        top_left,
        top_right,
        bottom_left,
        bottom_right
    } = distribute_and_normalize(RoundedRectangle {
        top_left_corner_radius,
        top_right_corner_radius,
        bottom_right_corner_radius,
        bottom_left_corner_radius,
        width,
        height,
    });

    get_svg_path_from_path_params(SvgPathInput {
        width,
        height,
        top_left_path_params: &get_path_params_for_corner(CornerParams {
            corner_radius: top_left.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: top_left.rounding_and_smoothing_budget
        }),
        top_right_path_params: &get_path_params_for_corner(CornerParams {
            corner_radius: top_right.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: top_right.rounding_and_smoothing_budget
        }),
        bottom_right_path_params: &get_path_params_for_corner(CornerParams {
            corner_radius: bottom_right.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: bottom_right.rounding_and_smoothing_budget
        }),
        bottom_left_path_params: &get_path_params_for_corner(CornerParams {
            corner_radius: bottom_left.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: bottom_left.rounding_and_smoothing_budget
        })
    })
}