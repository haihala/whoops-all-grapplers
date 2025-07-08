const PI = 3.14159265359;
const TAU = PI*2.0;

fn inverse_lerp(floor: f32, ceil: f32, val: f32) -> f32 {
    return (val - floor) / (ceil - floor);
}

// val assumed to be in range [curr_a, curr_b], wanted to be in range [new_a, new_b]
fn remap(val: f32, curr_a: f32, curr_b: f32, new_a: f32, new_b: f32) -> f32 {
    let norm = inverse_lerp(curr_a, curr_b, val);
    return norm * (new_b - new_a) + new_a;
}

// Easing functions from https://easings.net
// Did type them out by hand, they may have problems

fn easeInSine(x: f32) -> f32 {
    return 1.0 - cos(x * PI / 2.0);
}
fn easeOutSine(x: f32) -> f32 {
    return sin(x * PI / 2.0);
}
fn easeInOutSine(x: f32) -> f32 {
    return -(cos(PI*x) - 1.0) / 2.0;
}

fn easeInCubic(x: f32) -> f32 {
    return pow(x, 3.0);
}
fn easeOutCubic(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 3.0);
}
fn easeInOutCubic(x: f32) -> f32 {
    if (x < 0.5) {
        return 4.0 * pow(x, 3.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 3.0) / 2.0;
    }
}

fn easeInQuint(x: f32) -> f32{
    return pow(x, 5.0);
}
fn easeOutQuint(x: f32) -> f32 {
    return 1 - pow(1 - x, 5.0);
}
fn easeInOutQuint(x: f32) -> f32 {
    if (x < 0.5) {
        return 16 * pow(x, 5.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 5.0) / 2.0;
    }
}

fn easeInCirc(x: f32) -> f32 {
    return 1.0 - sqrt(1.0-pow(x, 2.0));
}
fn easeOutCirc(x: f32) -> f32 {
    return sqrt(1.0-pow(x-1, 2.0));
}
fn easeInOutCirc(x: f32) -> f32 {
    if (x < 0.5) {
        return (1-sqrt(1.0 - pow(2.0 * x, 2.0))) / 2.0;
    } else {
        return (sqrt(1.0 - pow(-2.0 * x + 2, 2.0)) + 1.0) / 2.0;
    }
}

fn easeInElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c4 = (2.0 * PI) / 3.0;
        return -pow(2.0, 10.0*x-10.0) * sin((x*10.0 - 10.75)*c4);
    }
}
fn easeOutElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c4 = (2.0 * PI) / 3.0;
        return pow(2.0, -10.0*x) * sin((x*10.0 - 0.75)*c4) + 1.0;
    }
}
fn easeInOutElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c5 = (2.0 * PI) / 4.5;
        if (x < 0.5) {
            return -pow(2.0, 20.0*x - 10.0) * sin((x*20.0 - 11.125)*c5) / 2.0;
        } else {
            return pow(2.0, -20.0*x + 10.0) * sin((x*20.0 - 11.125)*c5) / 2.0 + 1.0;
        }
    }
}

fn easeInQuad(x: f32) -> f32 {
    return pow(x, 2.0);
}
fn easeOutQuad(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 2.0);
}
fn easeInOutQuad(x: f32) -> f32 {
    if (x < 0.5) {
        return 2 * pow(x, 2.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 2.0) / 2.0;
    }
}

fn easeInQuart(x: f32) -> f32 {
    return pow(x, 4.0);
}
fn easeOutQuart(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 4.0);
}
fn easeInOutQuart(x: f32) -> f32 {
    if (x < 0.5) {
        return 8 * pow(x, 4.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 4.0) / 2.0;
    }
}

fn easeInExpo(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    }
    return pow(2.0, 10.0*x-10.0);
}
fn easeOutExpo(x: f32) -> f32 {
    if (x >= 1.0) {
        return 1.0;
    }
    return 1.0 - pow(2.0, -10.0*x);
}
fn easeInOutExpo(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        if (x < 0.5) {
            return pow(2.0, 20.0 * x - 10.0) / 2.0;
        } else {
            return 2.0 - pow(2.0, -20.0 * x + 10.0) / 2.0;
        }
    }
}

fn easeInBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return c3 * pow(x, 3.0) - c1 * pow(x, 2.0);
}
fn easeOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return 1+c3 * pow(x-1.0, 3.0) + c1 * pow(x-1, 2.0);
}
fn easeInOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    if (x < 0.5) {
        return (pow(2.0 * x, 2.0) * ((c2 + 1.0) * 2.0 * x - c2)) / 2.0;
    } else {
        return (pow(2.0 * x - 2.0, 2.0) * ((c2 + 1.0) * (x * 2.0 - 2.0) + c2) + 2.0) / 2.0;
    }
}

// Others are in, out, inout, but the in depends on the out here
// Not sure about this, as it used -= inline, and I'm assuming that evaluates
// To the value after the subtraction
fn easeOutBounce(x: f32) -> f32{
    let n1 = 7.5625;
    let d1 = 2.75;
    if (x < 1 / d1) {
        return n1 * pow(x, 2.0);
    } else if (x < 2 / d1) {
        let nx = x - 1.5;
        return n1 * (nx / d1) * nx + 0.75;
    } else if (x < 2.5 / d1) {
        let nx = x - 2.25;
        return n1 * (nx / d1) * nx + 0.9375;
    } else {
        let nx = x - 2.625;
        return n1 * (nx/ d1) * nx + 0.984375;
    }
}
fn easeInBounce(x: f32) -> f32{
    return 1.0 - easeOutBounce(1.0 - x);
}
fn easeInOutBounce(x: f32) -> f32{
    if (x < 0.5) {
        return (1.0 - easeOutBounce(1.0 - 2.0 * x)) / 2.0;
    } else {
        return (1.0 + easeOutBounce(2.0 * x - 1.0)) / 2.0;
    }
}

// From https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
// On generating random numbers, with help of y= [(a+x)sin(bx)] mod 1", W.J.J. Rey, 22nd European Meeting of Statisticians 1998
fn rand11(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }
fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }

