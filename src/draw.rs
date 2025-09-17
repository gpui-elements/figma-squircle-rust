pub struct CornerPathParams {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    p: f32,
    corner_radius: f32,
    arc_section_length: f32
}

pub struct CornerParams {
    pub corner_radius: f32,
    pub corner_smoothing: f32,
    pub preserve_smoothing: bool,
    pub rounding_and_smoothing_budget: f32
}

// The article from figma's blog
// https://www.figma.com/blog/desperately-seeking-squircles/
//
// The original code by MartinRGB
// https://github.com/MartinRGB/Figma_Squircles_Approximation/blob/bf29714aab58c54329f3ca130ffa16d39a2ff08c/js/rounded-corners.js#L64
pub fn get_path_params_for_corner(
    CornerParams {
        corner_radius,
        mut corner_smoothing,
        preserve_smoothing,
        rounding_and_smoothing_budget
    }: CornerParams
) -> CornerPathParams {
    // From figure 12.2 in the article
    // p = (1 + corner_smoothing) * q
    // in this case q = R because theta = 90deg
    let mut p = (1.0 + corner_smoothing) * corner_radius;

    // When there's not enough space left (p > rounding_and_smoothing_budget), there are 2 options:
    //
    // 1. What figma's currently doing: limit the smoothing value to make sure p <= rounding_and_smoothing_budget
    // But what this means is that at some point when cornerRadius is large enough,
    // increasing the smoothing value wouldn't do anything
    //
    // 2. Keep the original smoothing value and use it to calculate the bezier curve normally,
    // then adjust the control points to achieve similar curvature profile
    //
    // preserve_smoothing is a new option I added
    //
    // If preserve_smoothing is on then we'll just keep using the original smoothing value
    // and adjust the bezier curve later
    if !preserve_smoothing {
        let max_corner_smoothing = rounding_and_smoothing_budget / corner_radius - 1.0;
        corner_smoothing = corner_smoothing.min(max_corner_smoothing);
        p = p.min(rounding_and_smoothing_budget);
    }

    // In a normal rounded rectangle (corner_smoothing = 0), this is 90
    // The larger the smoothing, the smaller the arc
    let arc_measure = 90.0 * (1.0 - corner_smoothing);
    let arc_section_length =
        (arc_measure / 2.0).to_radians().sin() * corner_radius * std::f32::consts::SQRT_2;

    // In the article this is the distance between 2 control points: P3 and P4
    let angle_alpha = (90.0 - arc_measure) / 2.0;
    let p3_to_p4_distance = corner_radius * (angle_alpha / 2.0).to_radians().tan();

    // a, b, c and d are from figure 11.1 in the article
    let angle_beta = 45.0 * corner_smoothing;
    let c = p3_to_p4_distance *  angle_beta.to_radians().cos();
    let d = c * angle_beta.to_radians().tan();

    let mut b = (p - arc_section_length - c - d) / 3.0;
    let mut a = 2.0 * b;

    if preserve_smoothing && p > rounding_and_smoothing_budget {
        let p1_to_p3_max_distance =
            rounding_and_smoothing_budget - d - arc_section_length - c;

        // Try to maintain some distance between P1 and P2 so the curve wouldn't look weird
        let min_a = p1_to_p3_max_distance / 6.0;
        let max_b = p1_to_p3_max_distance - min_a;

        b = b.min(max_b);
        a = p1_to_p3_max_distance - b;
        p = p.min(rounding_and_smoothing_budget);
    }

    CornerPathParams {
        a,
        b,
        c,
        d,
        p,
        corner_radius,
        arc_section_length
    }
}

pub struct SvgPathInput<'a> {
    pub width: f32,
    pub height: f32,
    pub top_left_path_params: &'a CornerPathParams,
    pub top_right_path_params: &'a CornerPathParams,
    pub bottom_right_path_params: &'a CornerPathParams,
    pub bottom_left_path_params: &'a CornerPathParams,
}

pub fn get_svg_path_from_path_params(
    SvgPathInput {
        width,
        height,
        top_right_path_params,
        bottom_right_path_params,
        bottom_left_path_params,
        top_left_path_params
    }: SvgPathInput
) -> String {
    return format!(
        "M {} 0 {} L {} {} {} L {} {} {} L 0 {} {} Z",
        width - top_right_path_params.p,
        draw_top_right_path(top_right_path_params),
        width, height - bottom_right_path_params.p,
        draw_bottom_right_path(bottom_right_path_params),
        bottom_left_path_params.p, height,
        draw_bottom_left_path(bottom_left_path_params),
        top_left_path_params.p,
        draw_top_left_path(top_left_path_params)
    )
}

fn draw_top_right_path(CornerPathParams {
    a,
    b,
    c,
    d,
    p: _p,
    corner_radius,
    arc_section_length,
}: &CornerPathParams) -> String {
    if corner_radius != &0.0 {
        format!(
            "c {:.4} 0 {:.4} 0 {:.4} {:.4} a {:.4} {:.4} 0 0 1 {:.4} {:.4} c {:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            a, a + b, a + b + c, d,
            corner_radius, corner_radius, arc_section_length, arc_section_length,
            d, c,
            d, b + c,
            d, a + b + c
        )
    } else {
        //format!("1 {:.4} 0", p)
        String::new()
    }
}

fn draw_bottom_right_path(CornerPathParams {
    a,
    b,
    c,
    d,
    p: _p,
    corner_radius,
    arc_section_length,
}: &CornerPathParams) -> String {
    if corner_radius != &0.0 {
        format!(
            "c 0 {:.4} 0 {:.4} {:.4} {:.4} a {:.4} {:.4} 0 0 1 -{:.4} {:.4} c {:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            a,
            a + b,
            -d, a + b + c,
            corner_radius, corner_radius, arc_section_length, arc_section_length,
            -c, d,
            -(b + c), d,
            -(a + b + c), d
        )
    } else {
        String::new()
        //format!("1 0 {:.4}", p)
    }
}

fn draw_bottom_left_path(CornerPathParams {
    a,
    b,
    c,
    d,
    p: _p,
    corner_radius,
    arc_section_length,
}: &CornerPathParams) -> String {
    if corner_radius != &0.0 {
        format!(
            "c {:.4} 0 {:.4} 0 {:.4} {:.4} a {:.4} {:.4} 0 0 1 -{:.4} -{:.4} c {:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            -a,
            -(a + b),
            -(a + b + c), -d,
            corner_radius, corner_radius, arc_section_length, arc_section_length,
            -d, -c,
            -d, -(b + c),
            -d, -(a + b + c)
        )
    } else {
        String::new()
        //format!("1 {:.4} 0", -p)
    }
}

fn draw_top_left_path(CornerPathParams {
    a,
    b,
    c,
    d,
    p: _p,
    corner_radius,
    arc_section_length,
}: &CornerPathParams) -> String {
    if corner_radius != &0.0 {
        format!(
            "c 0 {:.4} 0 {:.4} {:.4} {:.4} a {:.4} {:.4} 0 0 1 {:.4} -{:.4} c {:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            -a,
            -(a + b),
            d, -(a + b + c),
            corner_radius, corner_radius, arc_section_length, arc_section_length,
            c, -d,
            b + c, -d,
            a + b + c, -d
        )
    } else {
        String::new()
        //format!("1 0 {:.4}", -p)
    }
}