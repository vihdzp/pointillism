//! We initialize constants for [`Freq`] and [`MidiNote`]. For instance,
//! [`Freq::A4`] = 440 Hz.
//!
//! Each name is made out of a pitch letter, followed by an optional `S` or `B`
//! for sharps and flats, followed by the octave number. We use `N1` for `-1`.
//!
//! Enharmonic notes are given their individual constant names, for good
//! measure.
//!
//! We only implement the notes from octaves -1 to 10, as anything lower is
//! unsupported as a [`MidiNote`], and anything higher is too high-pitched to be
//! practical. This range well-covers the human hearing range.
//!
//! This will hopefully be replaced with some macro code eventually.

// In case we decide to swap back to `f64`.
#![allow(clippy::excessive_precision)]
#![allow(clippy::unreadable_literal)]

use super::{Freq, MidiNote};

impl MidiNote {
    // OCTAVE -1

    /// The note C-1.
    pub const CN1: Self = Self::new(0);

    /// The note C#-1.
    pub const CSN1: Self = Self::new(1);
    /// The note Db-1.
    pub const DBN1: Self = Self::CSN1;

    /// The note D-1.
    pub const DN1: Self = Self::new(2);

    /// The note D#-1.
    pub const DSN1: Self = Self::new(3);
    /// The note Eb-1.
    pub const EBN1: Self = Self::DSN1;

    /// The note E-1.
    pub const EN1: Self = Self::new(4);
    /// The note Fb-1.
    pub const FBN1: Self = Self::DSN1;

    /// The note F-1.
    pub const FN1: Self = Self::new(5);
    /// The note E#-1.
    pub const ESN1: Self = Self::EN1;

    /// The note F#-1.
    pub const FSN1: Self = Self::new(6);
    /// The note Gb-1.
    pub const GBN1: Self = Self::FSN1;

    /// The note G-1.
    pub const GN1: Self = Self::new(7);

    /// The note G#-1.
    pub const GSN1: Self = Self::new(8);
    /// The note Ab-1.
    pub const ABN1: Self = Self::GSN1;

    /// The note A-1.
    pub const AN1: Self = Self::new(9);

    /// The note A#-1.
    pub const ASN1: Self = Self::new(10);
    /// The note Bb-1.
    pub const BBN1: Self = Self::ASN1;

    /// The note B-1.
    pub const BN1: Self = Self::new(11);
    /// The note Cb0.
    pub const CB0: Self = Self::BN1;

    // OCTAVE 0

    /// The note C0.
    pub const C0: Self = Self::new(12);

    /// The note C#0.
    pub const CS0: Self = Self::new(13);
    /// The note Db0.
    pub const DB0: Self = Self::CS0;

    /// The note D0.
    pub const D0: Self = Self::new(14);

    /// The note D#0.
    pub const DS0: Self = Self::new(15);
    /// The note Eb0.
    pub const EB0: Self = Self::DS0;

    /// The note E0.
    pub const E0: Self = Self::new(16);
    /// The note Fb0.
    pub const FB0: Self = Self::DS0;

    /// The note F0.
    pub const F0: Self = Self::new(17);
    /// The note E#0.
    pub const ES0: Self = Self::E0;

    /// The note F#0.
    pub const FS0: Self = Self::new(18);
    /// The note Gb0.
    pub const GB0: Self = Self::FS0;

    /// The note G0.
    pub const G0: Self = Self::new(19);

    /// The note G#0.
    pub const GS0: Self = Self::new(20);
    /// The note Ab0.
    pub const AB0: Self = Self::GS0;

    /// The note A0.
    pub const A0: Self = Self::new(21);

    /// The note A#0.
    pub const AS0: Self = Self::new(22);
    /// The note Bb0.
    pub const BB0: Self = Self::AS0;

    /// The note B0.
    pub const B0: Self = Self::new(23);
    /// The note Cb1.
    pub const CB1: Self = Self::B0;

    // OCTAVE 1

    /// The note C1.
    pub const C1: Self = Self::new(24);

    /// The note C#1.
    pub const CS1: Self = Self::new(25);
    /// The note Db1.
    pub const DB1: Self = Self::CS1;

    /// The note D1.
    pub const D1: Self = Self::new(26);

    /// The note D#1.
    pub const DS1: Self = Self::new(27);
    /// The note Eb1.
    pub const EB1: Self = Self::DS1;

    /// The note E1.
    pub const E1: Self = Self::new(28);
    /// The note Fb1.
    pub const FB1: Self = Self::DS1;

    /// The note F1.
    pub const F1: Self = Self::new(29);
    /// The note E#1.
    pub const ES1: Self = Self::E1;

    /// The note F#1.
    pub const FS1: Self = Self::new(30);
    /// The note Gb1.
    pub const GB1: Self = Self::FS1;

    /// The note G1.
    pub const G1: Self = Self::new(31);

    /// The note G#1.
    pub const GS1: Self = Self::new(32);
    /// The note Ab1.
    pub const AB1: Self = Self::GS1;

    /// The note A1.
    pub const A1: Self = Self::new(33);

    /// The note A#1.
    pub const AS1: Self = Self::new(34);
    /// The note Bb1.
    pub const BB1: Self = Self::AS1;

    /// The note B1.
    pub const B1: Self = Self::new(35);
    /// The note Cb2.
    pub const CB2: Self = Self::B1;

    // OCTAVE 2

    /// The note C2.
    pub const C2: Self = Self::new(36);

    /// The note C#2.
    pub const CS2: Self = Self::new(37);
    /// The note Db2.
    pub const DB2: Self = Self::CS2;

    /// The note D2.
    pub const D2: Self = Self::new(38);

    /// The note D#2.
    pub const DS2: Self = Self::new(39);
    /// The note Eb2.
    pub const EB2: Self = Self::DS2;

    /// The note E2.
    pub const E2: Self = Self::new(40);
    /// The note Fb2.
    pub const FB2: Self = Self::DS2;

    /// The note F2.
    pub const F2: Self = Self::new(41);
    /// The note E#2.
    pub const ES2: Self = Self::E2;

    /// The note F#2.
    pub const FS2: Self = Self::new(42);
    /// The note Gb2.
    pub const GB2: Self = Self::FS2;

    /// The note G2.
    pub const G2: Self = Self::new(43);

    /// The note G#2.
    pub const GS2: Self = Self::new(44);
    /// The note Ab2.
    pub const AB2: Self = Self::GS2;

    /// The note A2.
    pub const A2: Self = Self::new(45);

    /// The note A#2.
    pub const AS2: Self = Self::new(46);
    /// The note Bb2.
    pub const BB2: Self = Self::AS2;

    /// The note B2.
    pub const B2: Self = Self::new(47);
    /// The note Cb3.
    pub const CB3: Self = Self::B2;

    // OCTAVE 3

    /// The note C3.
    pub const C3: Self = Self::new(48);

    /// The note C#3.
    pub const CS3: Self = Self::new(49);
    /// The note Db3.
    pub const DB3: Self = Self::CS3;

    /// The note D3.
    pub const D3: Self = Self::new(50);

    /// The note D#3.
    pub const DS3: Self = Self::new(51);
    /// The note Eb3.
    pub const EB3: Self = Self::DS3;

    /// The note E3.
    pub const E3: Self = Self::new(52);
    /// The note Fb3.
    pub const FB3: Self = Self::DS3;

    /// The note F3.
    pub const F3: Self = Self::new(53);
    /// The note E#3.
    pub const ES3: Self = Self::E3;

    /// The note F#3.
    pub const FS3: Self = Self::new(54);
    /// The note Gb3.
    pub const GB3: Self = Self::FS3;

    /// The note G3.
    pub const G3: Self = Self::new(55);

    /// The note G#3.
    pub const GS3: Self = Self::new(56);
    /// The note Ab3.
    pub const AB3: Self = Self::GS3;

    /// The note A3.
    pub const A3: Self = Self::new(57);

    /// The note A#3.
    pub const AS3: Self = Self::new(58);
    /// The note Bb3.
    pub const BB3: Self = Self::AS3;

    /// The note B3.
    pub const B3: Self = Self::new(59);
    /// The note Cb4.
    pub const CB4: Self = Self::B3;

    // OCTAVE 4

    /// The note C4.
    pub const C4: Self = Self::new(60);

    /// The note C#4.
    pub const CS4: Self = Self::new(61);
    /// The note Db4.
    pub const DB4: Self = Self::CS4;

    /// The note D4.
    pub const D4: Self = Self::new(62);

    /// The note D#4.
    pub const DS4: Self = Self::new(63);
    /// The note Eb4.
    pub const EB4: Self = Self::DS4;

    /// The note E4.
    pub const E4: Self = Self::new(64);
    /// The note Fb4.
    pub const FB4: Self = Self::DS4;

    /// The note F4.
    pub const F4: Self = Self::new(65);
    /// The note E#4.
    pub const ES4: Self = Self::E4;

    /// The note F#4.
    pub const FS4: Self = Self::new(66);
    /// The note Gb4.
    pub const GB4: Self = Self::FS4;

    /// The note G4.
    pub const G4: Self = Self::new(67);

    /// The note G#4.
    pub const GS4: Self = Self::new(68);
    /// The note Ab4.
    pub const AB4: Self = Self::GS4;

    /// The note A4.
    pub const A4: Self = Self::new(69);

    /// The note A#4.
    pub const AS4: Self = Self::new(70);
    /// The note Bb4.
    pub const BB4: Self = Self::AS4;

    /// The note B4.
    pub const B4: Self = Self::new(71);
    /// The note Cb5.
    pub const CB5: Self = Self::B4;

    // OCTAVE 5

    /// The note C5.
    pub const C5: Self = Self::new(72);

    /// The note C#5.
    pub const CS5: Self = Self::new(73);
    /// The note Db5.
    pub const DB5: Self = Self::CS5;

    /// The note D5.
    pub const D5: Self = Self::new(74);

    /// The note D#5.
    pub const DS5: Self = Self::new(75);
    /// The note Eb5.
    pub const EB5: Self = Self::DS5;

    /// The note E5.
    pub const E5: Self = Self::new(76);
    /// The note Fb5.
    pub const FB5: Self = Self::DS5;

    /// The note F5.
    pub const F5: Self = Self::new(77);
    /// The note E#5.
    pub const ES5: Self = Self::E5;

    /// The note F#5.
    pub const FS5: Self = Self::new(78);
    /// The note Gb5.
    pub const GB5: Self = Self::FS5;

    /// The note G5.
    pub const G5: Self = Self::new(79);

    /// The note G#5.
    pub const GS5: Self = Self::new(80);
    /// The note Ab5.
    pub const AB5: Self = Self::GS5;

    /// The note A5.
    pub const A5: Self = Self::new(81);

    /// The note A#5.
    pub const AS5: Self = Self::new(82);
    /// The note Bb5.
    pub const BB5: Self = Self::AS5;

    /// The note B5.
    pub const B5: Self = Self::new(83);
    /// The note Cb6.
    pub const CB6: Self = Self::B5;

    // OCTAVE 6

    /// The note C6.
    pub const C6: Self = Self::new(84);

    /// The note C#6.
    pub const CS6: Self = Self::new(85);
    /// The note Db6.
    pub const DB6: Self = Self::CS6;

    /// The note D6.
    pub const D6: Self = Self::new(86);

    /// The note D#6.
    pub const DS6: Self = Self::new(87);
    /// The note Eb6.
    pub const EB6: Self = Self::DS6;

    /// The note E6.
    pub const E6: Self = Self::new(88);
    /// The note Fb6.
    pub const FB6: Self = Self::DS6;

    /// The note F6.
    pub const F6: Self = Self::new(89);
    /// The note E#6.
    pub const ES6: Self = Self::E6;

    /// The note F#6.
    pub const FS6: Self = Self::new(90);
    /// The note Gb6.
    pub const GB6: Self = Self::FS6;

    /// The note G6.
    pub const G6: Self = Self::new(91);

    /// The note G#6.
    pub const GS6: Self = Self::new(92);
    /// The note Ab6.
    pub const AB6: Self = Self::GS6;

    /// The note A6.
    pub const A6: Self = Self::new(93);

    /// The note A#6.
    pub const AS6: Self = Self::new(94);
    /// The note Bb6.
    pub const BB6: Self = Self::AS6;

    /// The note B6.
    pub const B6: Self = Self::new(95);
    /// The note Cb7.
    pub const CB7: Self = Self::B6;

    // OCTAVE 7

    /// The note C7.
    pub const C7: Self = Self::new(96);

    /// The note C#7.
    pub const CS7: Self = Self::new(97);
    /// The note Db7.
    pub const DB7: Self = Self::CS7;

    /// The note D7.
    pub const D7: Self = Self::new(98);

    /// The note D#7.
    pub const DS7: Self = Self::new(99);
    /// The note Eb7.
    pub const EB7: Self = Self::DS7;

    /// The note E7.
    pub const E7: Self = Self::new(100);
    /// The note Fb7.
    pub const FB7: Self = Self::DS7;

    /// The note F7.
    pub const F7: Self = Self::new(101);
    /// The note E#7.
    pub const ES7: Self = Self::E7;

    /// The note F#7.
    pub const FS7: Self = Self::new(102);
    /// The note Gb7.
    pub const GB7: Self = Self::FS7;

    /// The note G7.
    pub const G7: Self = Self::new(103);

    /// The note G#7.
    pub const GS7: Self = Self::new(104);
    /// The note Ab7.
    pub const AB7: Self = Self::GS7;

    /// The note A7.
    pub const A7: Self = Self::new(105);

    /// The note A#7.
    pub const AS7: Self = Self::new(106);
    /// The note Bb7.
    pub const BB7: Self = Self::AS7;

    /// The note B7.
    pub const B7: Self = Self::new(107);
    /// The note Cb8.
    pub const CB8: Self = Self::B7;

    // OCTAVE 8

    /// The note C8.
    pub const C8: Self = Self::new(108);

    /// The note C#8.
    pub const CS8: Self = Self::new(109);
    /// The note Db8.
    pub const DB8: Self = Self::CS8;

    /// The note D8.
    pub const D8: Self = Self::new(110);

    /// The note D#8.
    pub const DS8: Self = Self::new(111);
    /// The note Eb8.
    pub const EB8: Self = Self::DS8;

    /// The note E8.
    pub const E8: Self = Self::new(112);
    /// The note Fb8.
    pub const FB8: Self = Self::DS8;

    /// The note F8.
    pub const F8: Self = Self::new(113);
    /// The note E#8.
    pub const ES8: Self = Self::E8;

    /// The note F#8.
    pub const FS8: Self = Self::new(114);
    /// The note Gb8.
    pub const GB8: Self = Self::FS8;

    /// The note G8.
    pub const G8: Self = Self::new(115);

    /// The note G#8.
    pub const GS8: Self = Self::new(116);
    /// The note Ab8.
    pub const AB8: Self = Self::GS8;

    /// The note A8.
    pub const A8: Self = Self::new(117);

    /// The note A#8.
    pub const AS8: Self = Self::new(118);
    /// The note Bb8.
    pub const BB8: Self = Self::AS8;

    /// The note B8.
    pub const B8: Self = Self::new(119);
    /// The note Cb9.
    pub const CB9: Self = Self::B8;

    // OCTAVE 9

    /// The note C9.
    pub const C9: Self = Self::new(120);

    /// The note C#9.
    pub const CS9: Self = Self::new(121);
    /// The note Db9.
    pub const DB9: Self = Self::CS9;

    /// The note D9.
    pub const D9: Self = Self::new(122);

    /// The note D#9.
    pub const DS9: Self = Self::new(123);
    /// The note Eb9.
    pub const EB9: Self = Self::DS9;

    /// The note E9.
    pub const E9: Self = Self::new(124);
    /// The note Fb9.
    pub const FB9: Self = Self::DS9;

    /// The note F9.
    pub const F9: Self = Self::new(125);
    /// The note E#9.
    pub const ES9: Self = Self::E9;

    /// The note F#9.
    pub const FS9: Self = Self::new(126);
    /// The note Gb9.
    pub const GB9: Self = Self::FS9;

    /// The note G9.
    pub const G9: Self = Self::new(127);

    /// The note G#9.
    pub const GS9: Self = Self::new(128);
    /// The note Ab9.
    pub const AB9: Self = Self::GS9;

    /// The note A9.
    pub const A9: Self = Self::new(129);

    /// The note A#9.
    pub const AS9: Self = Self::new(130);
    /// The note Bb9.
    pub const BB9: Self = Self::AS9;

    /// The note B9.
    pub const B9: Self = Self::new(131);
    /// The note Cb10.
    pub const CB10: Self = Self::B9;

    // OCTAVE 10

    /// The note C10.
    pub const C10: Self = Self::new(132);

    /// The note C#10.
    pub const CS10: Self = Self::new(133);
    /// The note Db10.
    pub const DB10: Self = Self::CS10;

    /// The note D10.
    pub const D10: Self = Self::new(134);

    /// The note D#10.
    pub const DS10: Self = Self::new(135);
    /// The note Eb10.
    pub const EB10: Self = Self::DS10;

    /// The note E10.
    pub const E10: Self = Self::new(136);
    /// The note Fb10.
    pub const FB10: Self = Self::DS10;

    /// The note F10.
    pub const F10: Self = Self::new(137);
    /// The note E#10.
    pub const ES10: Self = Self::E10;

    /// The note F#10.
    pub const FS10: Self = Self::new(138);
    /// The note Gb10.
    pub const GB10: Self = Self::FS10;

    /// The note G10.
    pub const G10: Self = Self::new(139);

    /// The note G#10.
    pub const GS10: Self = Self::new(140);
    /// The note Ab10.
    pub const AB10: Self = Self::GS10;

    /// The note A10.
    pub const A10: Self = Self::new(141);

    /// The note A#10.
    pub const AS10: Self = Self::new(142);
    /// The note Bb10.
    pub const BB10: Self = Self::AS10;

    /// The note B10.
    pub const B10: Self = Self::new(143);
    /// The note Cb11.
    pub const CB11: Self = Self::B10;
}

impl Freq {
    // OCTAVE -1

    /// The note C-1.
    pub const CN1: Self = Self::new(8.1757989156437073336828122976033);

    /// The note C#-1.
    pub const CSN1: Self = Self::new(8.6619572180272530077745729250378);
    /// The note Db-1.
    pub const DBN1: Self = Self::CSN1;

    /// The note D-1.
    pub const DN1: Self = Self::new(9.1770239974189862582119691831431);

    /// The note D#-1.
    pub const DSN1: Self = Self::new(9.7227182413150284605116099789417);
    /// The note Eb-1.
    pub const EBN1: Self = Self::DSN1;

    /// The note E-1.
    pub const EN1: Self = Self::new(10.300861153527185304245055032705);
    /// The note Fb-1.
    pub const FBN1: Self = Self::DSN1;

    /// The note F-1.
    pub const FN1: Self = Self::new(10.913382232281371388917976269997);
    /// The note E#-1.
    pub const ESN1: Self = Self::EN1;

    /// The note F#-1.
    pub const FSN1: Self = Self::new(11.562325709738574966677975298207);
    /// The note Gb-1.
    pub const GBN1: Self = Self::FSN1;

    /// The note G-1.
    pub const GN1: Self = Self::new(12.24985737442966544017811032687);

    /// The note G#-1.
    pub const GSN1: Self = Self::new(12.978271799373285578826305904179);
    /// The note Ab-1.
    pub const ABN1: Self = Self::GSN1;

    /// The note A-1.
    pub const AN1: Self = Self::new(13.75);

    /// The note A#-1.
    pub const ASN1: Self = Self::new(14.567617547440309887725097805512);
    /// The note Bb-1.
    pub const BBN1: Self = Self::ASN1;

    /// The note B-1.
    pub const BN1: Self = Self::new(15.433853164253878494711079433089);
    /// The note Cb0.
    pub const CB0: Self = Self::BN1;

    // OCTAVE 0

    /// The note C0.
    pub const C0: Self = Self::new(16.351597831287414667365624595207);

    /// The note C#0.
    pub const CS0: Self = Self::new(17.323914436054506015549145850076);
    /// The note Db0.
    pub const DB0: Self = Self::CS0;

    /// The note D0.
    pub const D0: Self = Self::new(18.354047994837972516423938366286);

    /// The note D#0.
    pub const DS0: Self = Self::new(19.445436482630056921023219957883);
    /// The note Eb0.
    pub const EB0: Self = Self::DS0;

    /// The note E0.
    pub const E0: Self = Self::new(20.60172230705437060849011006541);
    /// The note Fb0.
    pub const FB0: Self = Self::DS0;

    /// The note F0.
    pub const F0: Self = Self::new(21.826764464562742777835952539994);
    /// The note E#0.
    pub const ES0: Self = Self::E0;

    /// The note F#0.
    pub const FS0: Self = Self::new(23.124651419477149933355950596413);
    /// The note Gb0.
    pub const GB0: Self = Self::FS0;

    /// The note G0.
    pub const G0: Self = Self::new(24.499714748859330880356220653739);

    /// The note G#0.
    pub const GS0: Self = Self::new(25.956543598746571157652611808357);
    /// The note Ab0.
    pub const AB0: Self = Self::GS0;

    /// The note A0.
    pub const A0: Self = Self::new(27.5);

    /// The note A#0.
    pub const AS0: Self = Self::new(29.135235094880619775450195611024);
    /// The note Bb0.
    pub const BB0: Self = Self::AS0;

    /// The note B0.
    pub const B0: Self = Self::new(30.867706328507756989422158866177);
    /// The note Cb1.
    pub const CB1: Self = Self::B0;

    // OCTAVE 1

    /// The note C1.
    pub const C1: Self = Self::new(32.703195662574829334731249190413);

    /// The note C#1.
    pub const CS1: Self = Self::new(34.647828872109012031098291700151);
    /// The note Db1.
    pub const DB1: Self = Self::CS1;

    /// The note D1.
    pub const D1: Self = Self::new(36.708095989675945032847876732572);

    /// The note D#1.
    pub const DS1: Self = Self::new(38.890872965260113842046439915767);
    /// The note Eb1.
    pub const EB1: Self = Self::DS1;

    /// The note E1.
    pub const E1: Self = Self::new(41.203444614108741216980220130819);
    /// The note Fb1.
    pub const FB1: Self = Self::DS1;

    /// The note F1.
    pub const F1: Self = Self::new(43.653528929125485555671905079988);
    /// The note E#1.
    pub const ES1: Self = Self::E1;

    /// The note F#1.
    pub const FS1: Self = Self::new(46.249302838954299866711901192827);
    /// The note Gb1.
    pub const GB1: Self = Self::FS1;

    /// The note G1.
    pub const G1: Self = Self::new(48.999429497718661760712441307478);

    /// The note G#1.
    pub const GS1: Self = Self::new(51.913087197493142315305223616714);
    /// The note Ab1.
    pub const AB1: Self = Self::GS1;

    /// The note A1.
    pub const A1: Self = Self::new(55.0);

    /// The note A#1.
    pub const AS1: Self = Self::new(58.270470189761239550900391222049);
    /// The note Bb1.
    pub const BB1: Self = Self::AS1;

    /// The note B1.
    pub const B1: Self = Self::new(61.735412657015513978844317732355);
    /// The note Cb2.
    pub const CB2: Self = Self::B1;

    // OCTAVE 2

    /// The note C2.
    pub const C2: Self = Self::new(65.406391325149658669462498380826);

    /// The note C#2.
    pub const CS2: Self = Self::new(69.295657744218024062196583400303);
    /// The note Db2.
    pub const DB2: Self = Self::CS2;

    /// The note D2.
    pub const D2: Self = Self::new(73.416191979351890065695753465145);

    /// The note D#2.
    pub const DS2: Self = Self::new(77.781745930520227684092879831533);
    /// The note Eb2.
    pub const EB2: Self = Self::DS2;

    /// The note E2.
    pub const E2: Self = Self::new(82.406889228217482433960440261639);
    /// The note Fb2.
    pub const FB2: Self = Self::DS2;

    /// The note F2.
    pub const F2: Self = Self::new(87.307057858250971111343810159977);
    /// The note E#2.
    pub const ES2: Self = Self::E2;

    /// The note F#2.
    pub const FS2: Self = Self::new(92.498605677908599733423802385654);
    /// The note Gb2.
    pub const GB2: Self = Self::FS2;

    /// The note G2.
    pub const G2: Self = Self::new(97.998858995437323521424882614956);

    /// The note G#2.
    pub const GS2: Self = Self::new(103.82617439498628463061044723343);
    /// The note Ab2.
    pub const AB2: Self = Self::GS2;

    /// The note A2.
    pub const A2: Self = Self::new(110.0);

    /// The note A#2.
    pub const AS2: Self = Self::new(116.5409403795224791018007824441);
    /// The note Bb2.
    pub const BB2: Self = Self::AS2;

    /// The note B2.
    pub const B2: Self = Self::new(123.47082531403102795768863546471);
    /// The note Cb3.
    pub const CB3: Self = Self::B2;

    // OCTAVE 3

    /// The note C3.
    pub const C3: Self = Self::new(130.81278265029931733892499676165);

    /// The note C#3.
    pub const CS3: Self = Self::new(138.59131548843604812439316680061);
    /// The note Db3.
    pub const DB3: Self = Self::CS3;

    /// The note D3.
    pub const D3: Self = Self::new(146.83238395870378013139150693029);

    /// The note D#3.
    pub const DS3: Self = Self::new(155.56349186104045536818575966307);
    /// The note Eb3.
    pub const EB3: Self = Self::DS3;

    /// The note E3.
    pub const E3: Self = Self::new(164.81377845643496486792088052328);
    /// The note Fb3.
    pub const FB3: Self = Self::DS3;

    /// The note F3.
    pub const F3: Self = Self::new(174.61411571650194222268762031995);
    /// The note E#3.
    pub const ES3: Self = Self::E3;

    /// The note F#3.
    pub const FS3: Self = Self::new(184.99721135581719946684760477131);
    /// The note Gb3.
    pub const GB3: Self = Self::FS3;

    /// The note G3.
    pub const G3: Self = Self::new(195.99771799087464704284976522991);

    /// The note G#3.
    pub const GS3: Self = Self::new(207.65234878997256926122089446686);
    /// The note Ab3.
    pub const AB3: Self = Self::GS3;

    /// The note A3.
    pub const A3: Self = Self::new(220.0);

    /// The note A#3.
    pub const AS3: Self = Self::new(233.0818807590449582036015648882);
    /// The note Bb3.
    pub const BB3: Self = Self::AS3;

    /// The note B3.
    pub const B3: Self = Self::new(246.94165062806205591537727092942);
    /// The note Cb4.
    pub const CB4: Self = Self::B3;

    // OCTAVE 4

    /// The note C4.
    pub const C4: Self = Self::new(261.6255653005986346778499935233);

    /// The note C#4.
    pub const CS4: Self = Self::new(277.18263097687209624878633360121);
    /// The note Db4.
    pub const DB4: Self = Self::CS4;

    /// The note D4.
    pub const D4: Self = Self::new(293.66476791740756026278301386058);

    /// The note D#4.
    pub const DS4: Self = Self::new(311.12698372208091073637151932613);
    /// The note Eb4.
    pub const EB4: Self = Self::DS4;

    /// The note E4.
    pub const E4: Self = Self::new(329.62755691286992973584176104656);
    /// The note Fb4.
    pub const FB4: Self = Self::DS4;

    /// The note F4.
    pub const F4: Self = Self::new(349.22823143300388444537524063991);
    /// The note E#4.
    pub const ES4: Self = Self::E4;

    /// The note F#4.
    pub const FS4: Self = Self::new(369.99442271163439893369520954261);
    /// The note Gb4.
    pub const GB4: Self = Self::FS4;

    /// The note G4.
    pub const G4: Self = Self::new(391.99543598174929408569953045983);

    /// The note G#4.
    pub const GS4: Self = Self::new(415.30469757994513852244178893372);
    /// The note Ab4.
    pub const AB4: Self = Self::GS4;

    /// The note A4.
    pub const A4: Self = Self::new(440.0);

    /// The note A#4.
    pub const AS4: Self = Self::new(466.16376151808991640720312977639);
    /// The note Bb4.
    pub const BB4: Self = Self::AS4;

    /// The note B4.
    pub const B4: Self = Self::new(493.88330125612411183075454185884);
    /// The note Cb5.
    pub const CB5: Self = Self::B4;

    // OCTAVE 5

    /// The note C5.
    pub const C5: Self = Self::new(523.25113060119726935569998704661);

    /// The note C#5.
    pub const CS5: Self = Self::new(554.36526195374419249757266720242);
    /// The note Db5.
    pub const DB5: Self = Self::CS5;

    /// The note D5.
    pub const D5: Self = Self::new(587.32953583481512052556602772116);

    /// The note D#5.
    pub const DS5: Self = Self::new(622.25396744416182147274303865227);
    /// The note Eb5.
    pub const EB5: Self = Self::DS5;

    /// The note E5.
    pub const E5: Self = Self::new(659.25511382573985947168352209311);
    /// The note Fb5.
    pub const FB5: Self = Self::DS5;

    /// The note F5.
    pub const F5: Self = Self::new(698.45646286600776889075048127982);
    /// The note E#5.
    pub const ES5: Self = Self::E5;

    /// The note F#5.
    pub const FS5: Self = Self::new(739.98884542326879786739041908523);
    /// The note Gb5.
    pub const GB5: Self = Self::FS5;

    /// The note G5.
    pub const G5: Self = Self::new(783.99087196349858817139906091965);

    /// The note G#5.
    pub const GS5: Self = Self::new(830.60939515989027704488357786743);
    /// The note Ab5.
    pub const AB5: Self = Self::GS5;

    /// The note A5.
    pub const A5: Self = Self::new(880.0);

    /// The note A#5.
    pub const AS5: Self = Self::new(932.32752303617983281440625955278);
    /// The note Bb5.
    pub const BB5: Self = Self::AS5;

    /// The note B5.
    pub const B5: Self = Self::new(987.76660251224822366150908371768);
    /// The note Cb6.
    pub const CB6: Self = Self::B5;

    // OCTAVE 6

    /// The note C6.
    pub const C6: Self = Self::new(1046.5022612023945387113999740932);

    /// The note C#6.
    pub const CS6: Self = Self::new(1108.7305239074883849951453344048);
    /// The note Db6.
    pub const DB6: Self = Self::CS6;

    /// The note D6.
    pub const D6: Self = Self::new(1174.6590716696302410511320554423);

    /// The note D#6.
    pub const DS6: Self = Self::new(1244.5079348883236429454860773045);
    /// The note Eb6.
    pub const EB6: Self = Self::DS6;

    /// The note E6.
    pub const E6: Self = Self::new(1318.5102276514797189433670441862);
    /// The note Fb6.
    pub const FB6: Self = Self::DS6;

    /// The note F6.
    pub const F6: Self = Self::new(1396.9129257320155377815009625596);
    /// The note E#6.
    pub const ES6: Self = Self::E6;

    /// The note F#6.
    pub const FS6: Self = Self::new(1479.9776908465375957347808381705);
    /// The note Gb6.
    pub const GB6: Self = Self::FS6;

    /// The note G6.
    pub const G6: Self = Self::new(1567.9817439269971763427981218393);

    /// The note G#6.
    pub const GS6: Self = Self::new(1661.2187903197805540897671557349);
    /// The note Ab6.
    pub const AB6: Self = Self::GS6;

    /// The note A6.
    pub const A6: Self = Self::new(1760.0);

    /// The note A#6.
    pub const AS6: Self = Self::new(1864.6550460723596656288125191056);
    /// The note Bb6.
    pub const BB6: Self = Self::AS6;

    /// The note B6.
    pub const B6: Self = Self::new(1975.5332050244964473230181674354);
    /// The note Cb7.
    pub const CB7: Self = Self::B6;

    // OCTAVE 7

    /// The note C7.
    pub const C7: Self = Self::new(2093.0045224047890774227999481864);

    /// The note C#7.
    pub const CS7: Self = Self::new(2217.4610478149767699902906688097);
    /// The note Db7.
    pub const DB7: Self = Self::CS7;

    /// The note D7.
    pub const D7: Self = Self::new(2349.3181433392604821022641108846);

    /// The note D#7.
    pub const DS7: Self = Self::new(2489.0158697766472858909721546091);
    /// The note Eb7.
    pub const EB7: Self = Self::DS7;

    /// The note E7.
    pub const E7: Self = Self::new(2637.0204553029594378867340883724);
    /// The note Fb7.
    pub const FB7: Self = Self::DS7;

    /// The note F7.
    pub const F7: Self = Self::new(2793.8258514640310755630019251193);
    /// The note E#7.
    pub const ES7: Self = Self::E7;

    /// The note F#7.
    pub const FS7: Self = Self::new(2959.9553816930751914695616763409);
    /// The note Gb7.
    pub const GB7: Self = Self::FS7;

    /// The note G7.
    pub const G7: Self = Self::new(3135.9634878539943526855962436786);

    /// The note G#7.
    pub const GS7: Self = Self::new(3322.4375806395611081795343114697);
    /// The note Ab7.
    pub const AB7: Self = Self::GS7;

    /// The note A7.
    pub const A7: Self = Self::new(3520.0);

    /// The note A#7.
    pub const AS7: Self = Self::new(3729.3100921447193312576250382111);
    /// The note Bb7.
    pub const BB7: Self = Self::AS7;

    /// The note B7.
    pub const B7: Self = Self::new(3951.0664100489928946460363348707);
    /// The note Cb8.
    pub const CB8: Self = Self::B7;

    // OCTAVE 8

    /// The note C8.
    pub const C8: Self = Self::new(4186.0090448095781548455998963729);

    /// The note C#8.
    pub const CS8: Self = Self::new(4434.9220956299535399805813376194);
    /// The note Db8.
    pub const DB8: Self = Self::CS8;

    /// The note D8.
    pub const D8: Self = Self::new(4698.6362866785209642045282217693);

    /// The note D#8.
    pub const DS8: Self = Self::new(4978.0317395532945717819443092181);
    /// The note Eb8.
    pub const EB8: Self = Self::DS8;

    /// The note E8.
    pub const E8: Self = Self::new(5274.0409106059188757734681767449);
    /// The note Fb8.
    pub const FB8: Self = Self::DS8;

    /// The note F8.
    pub const F8: Self = Self::new(5587.6517029280621511260038502385);
    /// The note E#8.
    pub const ES8: Self = Self::E8;

    /// The note F#8.
    pub const FS8: Self = Self::new(5919.9107633861503829391233526818);
    /// The note Gb8.
    pub const GB8: Self = Self::FS8;

    /// The note G8.
    pub const G8: Self = Self::new(6271.9269757079887053711924873572);

    /// The note G#8.
    pub const GS8: Self = Self::new(6644.8751612791222163590686229394);
    /// The note Ab8.
    pub const AB8: Self = Self::GS8;

    /// The note A8.
    pub const A8: Self = Self::new(7040.0);

    /// The note A#8.
    pub const AS8: Self = Self::new(7458.6201842894386625152500764222);
    /// The note Bb8.
    pub const BB8: Self = Self::AS8;

    /// The note B8.
    pub const B8: Self = Self::new(7902.1328200979857892920726697414);
    /// The note Cb9.
    pub const CB9: Self = Self::B8;

    // OCTAVE 9

    /// The note C9.
    pub const C9: Self = Self::new(8372.0180896191563096911997927458);

    /// The note C#9.
    pub const CS9: Self = Self::new(8869.8441912599070799611626752387);
    /// The note Db9.
    pub const DB9: Self = Self::CS9;

    /// The note D9.
    pub const D9: Self = Self::new(9397.2725733570419284090564435385);

    /// The note D#9.
    pub const DS9: Self = Self::new(9956.0634791065891435638886184363);
    /// The note Eb9.
    pub const EB9: Self = Self::DS9;

    /// The note E9.
    pub const E9: Self = Self::new(10548.08182121183775154693635349);
    /// The note Fb9.
    pub const FB9: Self = Self::DS9;

    /// The note F9.
    pub const F9: Self = Self::new(11175.303405856124302252007700477);
    /// The note E#9.
    pub const ES9: Self = Self::E9;

    /// The note F#9.
    pub const FS9: Self = Self::new(11839.821526772300765878246705364);
    /// The note Gb9.
    pub const GB9: Self = Self::FS9;

    /// The note G9.
    pub const G9: Self = Self::new(12543.853951415977410742384974714);

    /// The note G#9.
    pub const GS9: Self = Self::new(13289.750322558244432718137245879);
    /// The note Ab9.
    pub const AB9: Self = Self::GS9;

    /// The note A9.
    pub const A9: Self = Self::new(14080.0);

    /// The note A#9.
    pub const AS9: Self = Self::new(14917.240368578877325030500152844);
    /// The note Bb9.
    pub const BB9: Self = Self::AS9;

    /// The note B9.
    pub const B9: Self = Self::new(15804.265640195971578584145339483);
    /// The note Cb10.
    pub const CB10: Self = Self::B9;

    // OCTAVE 10

    /// The note C10.
    pub const C10: Self = Self::new(16744.036179238312619382399585492);

    /// The note C#10.
    pub const CS10: Self = Self::new(17739.688382519814159922325350477);
    /// The note Db10.
    pub const DB10: Self = Self::CS10;

    /// The note D10.
    pub const D10: Self = Self::new(18794.545146714083856818112887077);

    /// The note D#10.
    pub const DS10: Self = Self::new(19912.126958213178287127777236873);
    /// The note Eb10.
    pub const EB10: Self = Self::DS10;

    /// The note E10.
    pub const E10: Self = Self::new(21096.16364242367550309387270698);
    /// The note Fb10.
    pub const FB10: Self = Self::DS10;

    /// The note F10.
    pub const F10: Self = Self::new(22350.606811712248604504015400954);
    /// The note E#10.
    pub const ES10: Self = Self::E10;

    /// The note F#10.
    pub const FS10: Self = Self::new(23679.643053544601531756493410727);
    /// The note Gb10.
    pub const GB10: Self = Self::FS10;

    /// The note G10.
    pub const G10: Self = Self::new(25087.707902831954821484769949429);

    /// The note G#10.
    pub const GS10: Self = Self::new(26579.500645116488865436274491758);
    /// The note Ab10.
    pub const AB10: Self = Self::GS10;

    /// The note A10.
    pub const A10: Self = Self::new(28160.0);

    /// The note A#10.
    pub const AS10: Self = Self::new(29834.480737157754650061000305689);
    /// The note Bb10.
    pub const BB10: Self = Self::AS10;

    /// The note B10.
    pub const B10: Self = Self::new(31608.531280391943157168290678966);
    /// The note Cb11.
    pub const CB11: Self = Self::B10;
}
