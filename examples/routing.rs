//! We route the same signal through two different effects, and recombine the results.

use pointillism::prelude::*;

fn main() {
    let mut signal = LoopGen::new(Saw, Freq::A3);
    let mut trem_env = LoopCurveGen::new(PosSaw, Freq::new(3.0));

    pointillism::create("examples/routing.wav", 5.0 * Time::SEC, |_| {
        let sgn1 = PwMapSgn::cubic(Ref::new(&signal));
        let sgn2 = Tremolo::new(Ref::new(&signal), trem_env.clone());
        let stereo = StereoMix::new(sgn1, sgn2);

        let res = stereo.get();
        signal.advance();
        trem_env.advance();
        res
    })
    .unwrap();
}
