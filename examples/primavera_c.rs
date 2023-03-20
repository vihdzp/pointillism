use pointillism::{generators::mix::StereoGen, prelude::*, signal::PointwiseMapSgn};

const BASE: Freq = Freq::new(222.2);
const FADE: Time = Time::new(20.0);
const VIB_TIME: Time = Time::new(40.0);
const MELODY_TIME: Time = Time::new(120.0);
const LENGTH: Time = Time::new(5.0 * 60.0);

fn fade(time: Time, length: Time, fade: Time) -> f64 {
    if time < fade {
        time.seconds / fade.seconds
    } else if time > length - fade {
        (length - time).seconds / fade.seconds
    } else {
        1.0
    }
}

fn binaural() -> impl Signal<Sample = Stereo> {
    // A sine wave.
    let wave = |freq| CurveGen::new(Sin::sin(), freq);

    // Vibrato sine wave.
    let vib = |freq| {
        Vibrato::new(
            wave(freq),
            freq,
            PointwiseMapSgn::new_pointwise(
                LoopCurveEnv::new(Sin::sin(), VIB_TIME.into()),
                Linear::rescale(-1.0, 1.0, 0.99, 1.01),
            ),
        )
    };

    // Binaural beats.
    StereoGen::new(wave(BASE * 0.985), vib(BASE))
}

fn melody() -> impl Signal<Sample = Mono> {
    let mut freq = 2.0 * A4;
    let notes = [3.0 / 2.0, 4.0 / 5.0, 4.0 / 3.0, 3.0 / 5.0];

    let wave = |freq| CurveGen::new(SawTri::tri(), freq);
    let shape = move |freq| {
        Envelope::new_generic(
            wave(freq),
            CurveEnv::new(PosSaw::new(), Time::new(5.0)),
            FnWrapper::new(|sgn: &mut CurveGen<SawTri>, val: f64| {
                sgn.curve_mut().shape = 1.0 - val.powf(0.2) / 2.0;
            }),
        )
    };
    let trem = move |freq| {
        StopTremolo::new(
            shape(freq),
            CurveEnv::new(PosInvSaw::new(), Time::new(10.0)),
        )
    };

    let poly = Polyphony::new();

    Loop::new(
        vec![Time::new(4.0)],
        poly,
        FnWrapper::new(move |poly: &mut Polyphony<_>, event: Event| {
            freq *= notes[event.idx % notes.len()];
            poly.add(trem(freq));
        }),
    )
}

fn main() {
    let mut binaural = binaural();
    let mut melody = melody();

    pointillism::create("examples/primavera_c.wav", LENGTH, |time| {
        let mut sample = binaural.next() * fade(time, LENGTH, FADE);

        if time > MELODY_TIME {
            sample += (melody.next() * fade(time - MELODY_TIME, LENGTH - MELODY_TIME, FADE))
                .duplicate()
                / 10.0;
        }

        sample / 2.0
    })
    .unwrap();
}
