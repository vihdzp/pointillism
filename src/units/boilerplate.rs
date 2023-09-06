//! We initialize constants for [`RawFreq`] and [`Note`]. For instance, [`RawFreq::A4`] = 440 Hz.
//!
//! Each name is made out of a pitch letter, followed by an optional `S` or `B` for sharps and
//! flats, followed by the octave number. We use `N` for a negative sign.
//!
//! Enharmonic notes are given their individual constant names, for good measure.
//!
//! We only implement the notes from octaves -1 to 10, as anything lower is unsupported as a
//! (standard) MIDI note, and anything higher is too high-pitched to be practical. This range
//! well-covers the human hearing range.
//!
//! This will hopefully be replaced with some macro code eventually.

use super::{freq::RawFreq, midi::MidiNote};

impl MidiNote {
    // OCTAVE -1

    /// The note B#-2.
    pub const BSN2: Self = Self::new(0);
    /// The note C-1.
    pub const CN1: Self = Self::BSN2;

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

    /// The note B#-1.
    pub const BSN1: Self = Self::new(12);
    /// The note C0.
    pub const C0: Self = Self::BSN1;

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

    /// The note B#0.
    pub const BS0: Self = Self::new(24);
    /// The note C1.
    pub const C1: Self = Self::BS0;

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

    /// The note B#1.
    pub const BS1: Self = Self::new(36);
    /// The note C2.
    pub const C2: Self = Self::BS1;

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

    /// The note B#2.
    pub const BS2: Self = Self::new(48);
    /// The note C3.
    pub const C3: Self = Self::BS2;

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

    /// The note B#3.
    pub const BS3: Self = Self::new(60);
    /// The note C4.
    pub const C4: Self = Self::BS3;

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

    /// The note B#4.
    pub const BS4: Self = Self::new(72);
    /// The note C5.
    pub const C5: Self = Self::BS4;

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

    /// The note B#5.
    pub const BS5: Self = Self::new(84);
    /// The note C6.
    pub const C6: Self = Self::BS5;

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

    /// The note B#6.
    pub const BS6: Self = Self::new(96);
    /// The note C7.
    pub const C7: Self = Self::BS6;

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

    /// The note B#7.
    pub const BS7: Self = Self::new(108);
    /// The note C8.
    pub const C8: Self = Self::BS7;

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
    pub const BS8: Self = Self::new(120);
    /// The note C9.
    pub const C9: Self = Self::BS8;

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

    /// The note B#9.
    pub const BS9: Self = Self::new(132);
    /// The note C10.
    pub const C10: Self = Self::BS9;

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

impl RawFreq {
    // OCTAVE -1

    /// The note B#-2.
    pub const BSN2: Self = Self::new(8.175_798_915_643_707);
    /// The note C-1.
    pub const CN1: Self = Self::BSN2;

    /// The note C#-1.
    pub const CSN1: Self = Self::new(8.661_957_218_027_252);
    /// The note Db-1.
    pub const DBN1: Self = Self::CSN1;

    /// The note D-1.
    pub const DN1: Self = Self::new(9.177_023_997_418_987);

    /// The note D#-1.
    pub const DSN1: Self = Self::new(9.722_718_241_315_029);
    /// The note Eb-1.
    pub const EBN1: Self = Self::DSN1;

    /// The note E-1.
    pub const EN1: Self = Self::new(10.300_861_153_527_185);
    /// The note Fb-1.
    pub const FBN1: Self = Self::DSN1;

    /// The note F-1.
    pub const FN1: Self = Self::new(10.913_382_232_281_371);
    /// The note E#-1.
    pub const ESN1: Self = Self::EN1;

    /// The note F#-1.
    pub const FSN1: Self = Self::new(11.562_325_709_738_575);
    /// The note Gb-1.
    pub const GBN1: Self = Self::FSN1;

    /// The note G-1.
    pub const GN1: Self = Self::new(12.249_857_374_429_665);

    /// The note G#-1.
    pub const GSN1: Self = Self::new(12.978_271_799_373_285);
    /// The note Ab-1.
    pub const ABN1: Self = Self::GSN1;

    /// The note A-1.
    pub const AN1: Self = Self::new(13.75);

    /// The note A#-1.
    pub const ASN1: Self = Self::new(14.567_617_547_440_31);
    /// The note Bb-1.
    pub const BBN1: Self = Self::ASN1;

    /// The note B-1.
    pub const BN1: Self = Self::new(15.433_853_164_253_879);
    /// The note Cb0.
    pub const CB0: Self = Self::BN1;

    // OCTAVE 0

    /// The note B#-1.
    pub const BSN1: Self = Self::new(16.351_597_831_287_414);
    /// The note C0.
    pub const C0: Self = Self::BSN1;

    /// The note C#0.
    pub const CS0: Self = Self::new(17.323_914_436_054_505);
    /// The note Db0.
    pub const DB0: Self = Self::CS0;

    /// The note D0.
    pub const D0: Self = Self::new(18.354_047_994_837_973);

    /// The note D#0.
    pub const DS0: Self = Self::new(19.445_436_482_630_058);
    /// The note Eb0.
    pub const EB0: Self = Self::DS0;

    /// The note E0.
    pub const E0: Self = Self::new(20.601_722_307_054_37);
    /// The note Fb0.
    pub const FB0: Self = Self::DS0;

    /// The note F0.
    pub const F0: Self = Self::new(21.826_764_464_562_743);
    /// The note E#0.
    pub const ES0: Self = Self::E0;

    /// The note F#0.
    pub const FS0: Self = Self::new(23.124_651_419_477_15);
    /// The note Gb0.
    pub const GB0: Self = Self::FS0;

    /// The note G0.
    pub const G0: Self = Self::new(24.499_714_748_859_33);

    /// The note G#0.
    pub const GS0: Self = Self::new(25.956_543_598_746_57);
    /// The note Ab0.
    pub const AB0: Self = Self::GS0;

    /// The note A0.
    pub const A0: Self = Self::new(27.5);

    /// The note A#0.
    pub const AS0: Self = Self::new(29.135_235_094_880_62);
    /// The note Bb0.
    pub const BB0: Self = Self::AS0;

    /// The note B0.
    pub const B0: Self = Self::new(30.867_706_328_507_758);
    /// The note Cb1.
    pub const CB1: Self = Self::B0;

    // OCTAVE 1

    /// The note B#0.
    pub const BS0: Self = Self::new(32.703_195_662_574_83);
    /// The note C1.
    pub const C1: Self = Self::BS0;

    /// The note C#1.
    pub const CS1: Self = Self::new(34.647_828_872_109_01);
    /// The note Db1.
    pub const DB1: Self = Self::CS1;

    /// The note D1.
    pub const D1: Self = Self::new(36.708_095_989_675_95);

    /// The note D#1.
    pub const DS1: Self = Self::new(38.890_872_965_260_115);
    /// The note Eb1.
    pub const EB1: Self = Self::DS1;

    /// The note E1.
    pub const E1: Self = Self::new(41.203_444_614_108_74);
    /// The note Fb1.
    pub const FB1: Self = Self::DS1;

    /// The note F1.
    pub const F1: Self = Self::new(43.653_528_929_125_486);
    /// The note E#1.
    pub const ES1: Self = Self::E1;

    /// The note F#1.
    pub const FS1: Self = Self::new(46.249_302_838_954_3);
    /// The note Gb1.
    pub const GB1: Self = Self::FS1;

    /// The note G1.
    pub const G1: Self = Self::new(48.999_429_497_718_66);

    /// The note G#1.
    pub const GS1: Self = Self::new(51.913_087_197_493_14);
    /// The note Ab1.
    pub const AB1: Self = Self::GS1;

    /// The note A1.
    pub const A1: Self = Self::new(55.0);

    /// The note A#1.
    pub const AS1: Self = Self::new(58.270_470_189_761_24);
    /// The note Bb1.
    pub const BB1: Self = Self::AS1;

    /// The note B1.
    pub const B1: Self = Self::new(61.735_412_657_015_516);
    /// The note Cb2.
    pub const CB2: Self = Self::B1;

    // OCTAVE 2

    /// The note B#1.
    pub const BS1: Self = Self::new(65.406_391_325_149_66);
    /// The note C2.
    pub const C2: Self = Self::BS1;

    /// The note C#2.
    pub const CS2: Self = Self::new(69.295_657_744_218_02);
    /// The note Db2.
    pub const DB2: Self = Self::CS2;

    /// The note D2.
    pub const D2: Self = Self::new(73.416_191_979_351_9);

    /// The note D#2.
    pub const DS2: Self = Self::new(77.781_745_930_520_23);
    /// The note Eb2.
    pub const EB2: Self = Self::DS2;

    /// The note E2.
    pub const E2: Self = Self::new(82.406_889_228_217_48);
    /// The note Fb2.
    pub const FB2: Self = Self::DS2;

    /// The note F2.
    pub const F2: Self = Self::new(87.307_057_858_250_97);
    /// The note E#2.
    pub const ES2: Self = Self::E2;

    /// The note F#2.
    pub const FS2: Self = Self::new(92.498_605_677_908_6);
    /// The note Gb2.
    pub const GB2: Self = Self::FS2;

    /// The note G2.
    pub const G2: Self = Self::new(97.998_858_995_437_32);

    /// The note G#2.
    pub const GS2: Self = Self::new(103.826_174_394_986_28);
    /// The note Ab2.
    pub const AB2: Self = Self::GS2;

    /// The note A2.
    pub const A2: Self = Self::new(110.0);

    /// The note A#2.
    pub const AS2: Self = Self::new(116.540_940_379_522_48);
    /// The note Bb2.
    pub const BB2: Self = Self::AS2;

    /// The note B2.
    pub const B2: Self = Self::new(123.470_825_314_031_03);
    /// The note Cb3.
    pub const CB3: Self = Self::B2;

    // OCTAVE 3

    /// The note B#2.
    pub const BS2: Self = Self::new(130.812_782_650_299_3);
    /// The note C3.
    pub const C3: Self = Self::BS2;

    /// The note C#3.
    pub const CS3: Self = Self::new(138.591_315_488_436_04);
    /// The note Db3.
    pub const DB3: Self = Self::CS3;

    /// The note D3.
    pub const D3: Self = Self::new(146.832_383_958_703_8);

    /// The note D#3.
    pub const DS3: Self = Self::new(155.563_491_861_040_46);
    /// The note Eb3.
    pub const EB3: Self = Self::DS3;

    /// The note E3.
    pub const E3: Self = Self::new(164.813_778_456_434_96);
    /// The note Fb3.
    pub const FB3: Self = Self::DS3;

    /// The note F3.
    pub const F3: Self = Self::new(174.614_115_716_501_94);
    /// The note E#3.
    pub const ES3: Self = Self::E3;

    /// The note F#3.
    pub const FS3: Self = Self::new(184.997_211_355_817_2);
    /// The note Gb3.
    pub const GB3: Self = Self::FS3;

    /// The note G3.
    pub const G3: Self = Self::new(195.997_717_990_874_63);

    /// The note G#3.
    pub const GS3: Self = Self::new(207.652_348_789_972_56);
    /// The note Ab3.
    pub const AB3: Self = Self::GS3;

    /// The note A3.
    pub const A3: Self = Self::new(220.0);

    /// The note A#3.
    pub const AS3: Self = Self::new(233.081_880_759_044_96);
    /// The note Bb3.
    pub const BB3: Self = Self::AS3;

    /// The note B3.
    pub const B3: Self = Self::new(246.941_650_628_062_06);
    /// The note Cb4.
    pub const CB4: Self = Self::B3;

    // OCTAVE 4

    /// The note B#3.
    pub const BS3: Self = Self::new(261.625_565_300_598_6);
    /// The note C4.
    pub const C4: Self = Self::BS3;

    /// The note C#4.
    pub const CS4: Self = Self::new(277.182_630_976_872_1);
    /// The note Db4.
    pub const DB4: Self = Self::CS4;

    /// The note D4.
    pub const D4: Self = Self::new(293.664_767_917_407_6);

    /// The note D#4.
    pub const DS4: Self = Self::new(311.126_983_722_080_9);
    /// The note Eb4.
    pub const EB4: Self = Self::DS4;

    /// The note E4.
    pub const E4: Self = Self::new(329.627_556_912_869_9);
    /// The note Fb4.
    pub const FB4: Self = Self::DS4;

    /// The note F4.
    pub const F4: Self = Self::new(349.228_231_433_003_9);
    /// The note E#4.
    pub const ES4: Self = Self::E4;

    /// The note F#4.
    pub const FS4: Self = Self::new(369.994_422_711_634_4);
    /// The note Gb4.
    pub const GB4: Self = Self::FS4;

    /// The note G4.
    pub const G4: Self = Self::new(391.995_435_981_749_27);

    /// The note G#4.
    pub const GS4: Self = Self::new(415.304_697_579_945_1);
    /// The note Ab4.
    pub const AB4: Self = Self::GS4;

    /// The note A4.
    pub const A4: Self = Self::new(440.0);

    /// The note A#4.
    pub const AS4: Self = Self::new(466.163_761_518_089_9);
    /// The note Bb4.
    pub const BB4: Self = Self::AS4;

    /// The note B4.
    pub const B4: Self = Self::new(493.883_301_256_124_1);
    /// The note Cb5.
    pub const CB5: Self = Self::B4;

    // OCTAVE 5

    /// The note B#4.
    pub const BS4: Self = Self::new(523.251_130_601_197_2);
    /// The note C5.
    pub const C5: Self = Self::BS4;

    /// The note C#5.
    pub const CS5: Self = Self::new(554.365_261_953_744_2);
    /// The note Db5.
    pub const DB5: Self = Self::CS5;

    /// The note D5.
    pub const D5: Self = Self::new(587.329_535_834_815_1);

    /// The note D#5.
    pub const DS5: Self = Self::new(622.253_967_444_161_8);
    /// The note Eb5.
    pub const EB5: Self = Self::DS5;

    /// The note E5.
    pub const E5: Self = Self::new(659.255_113_825_739_8);
    /// The note Fb5.
    pub const FB5: Self = Self::DS5;

    /// The note F5.
    pub const F5: Self = Self::new(698.456_462_866_007_8);
    /// The note E#5.
    pub const ES5: Self = Self::E5;

    /// The note F#5.
    pub const FS5: Self = Self::new(739.988_845_423_268_8);
    /// The note Gb5.
    pub const GB5: Self = Self::FS5;

    /// The note G5.
    pub const G5: Self = Self::new(783.990_871_963_498_5);

    /// The note G#5.
    pub const GS5: Self = Self::new(830.609_395_159_890_3);
    /// The note Ab5.
    pub const AB5: Self = Self::GS5;

    /// The note A5.
    pub const A5: Self = Self::new(880.0);

    /// The note A#5.
    pub const AS5: Self = Self::new(932.327_523_036_179_9);
    /// The note Bb5.
    pub const BB5: Self = Self::AS5;

    /// The note B5.
    pub const B5: Self = Self::new(987.766_602_512_248_3);
    /// The note Cb6.
    pub const CB6: Self = Self::B5;

    // OCTAVE 6

    /// The note B#5.
    pub const BS5: Self = Self::new(1_046.502_261_202_394_5);
    /// The note C6.
    pub const C6: Self = Self::BS5;

    /// The note C#6.
    pub const CS6: Self = Self::new(1_108.730_523_907_488_3);
    /// The note Db6.
    pub const DB6: Self = Self::CS6;

    /// The note D6.
    pub const D6: Self = Self::new(1_174.659_071_669_630_3);

    /// The note D#6.
    pub const DS6: Self = Self::new(1_244.507_934_888_323_7);
    /// The note Eb6.
    pub const EB6: Self = Self::DS6;

    /// The note E6.
    pub const E6: Self = Self::new(1_318.510_227_651_479_7);
    /// The note Fb6.
    pub const FB6: Self = Self::DS6;

    /// The note F6.
    pub const F6: Self = Self::new(1_396.912_925_732_015_5);
    /// The note E#6.
    pub const ES6: Self = Self::E6;

    /// The note F#6.
    pub const FS6: Self = Self::new(1_479.977_690_846_537_6);
    /// The note Gb6.
    pub const GB6: Self = Self::FS6;

    /// The note G6.
    pub const G6: Self = Self::new(1_567.981_743_926_997);

    /// The note G#6.
    pub const GS6: Self = Self::new(1_661.218_790_319_780_5);
    /// The note Ab6.
    pub const AB6: Self = Self::GS6;

    /// The note A6.
    pub const A6: Self = Self::new(1760.0);

    /// The note A#6.
    pub const AS6: Self = Self::new(1_864.655_046_072_359_7);
    /// The note Bb6.
    pub const BB6: Self = Self::AS6;

    /// The note B6.
    pub const B6: Self = Self::new(1_975.533_205_024_496_5);
    /// The note Cb7.
    pub const CB7: Self = Self::B6;

    // OCTAVE 7

    /// The note B#6.
    pub const BS6: Self = Self::new(2_093.004_522_404_789);
    /// The note C7.
    pub const C7: Self = Self::BS6;

    /// The note C#7.
    pub const CS7: Self = Self::new(2_217.461_047_814_976_6);
    /// The note Db7.
    pub const DB7: Self = Self::CS7;

    /// The note D7.
    pub const D7: Self = Self::new(2_349.318_143_339_260_6);

    /// The note D#7.
    pub const DS7: Self = Self::new(2_489.015_869_776_647_4);
    /// The note Eb7.
    pub const EB7: Self = Self::DS7;

    /// The note E7.
    pub const E7: Self = Self::new(2_637.020_455_302_959_4);
    /// The note Fb7.
    pub const FB7: Self = Self::DS7;

    /// The note F7.
    pub const F7: Self = Self::new(2_793.825_851_464_031);
    /// The note E#7.
    pub const ES7: Self = Self::E7;

    /// The note F#7.
    pub const FS7: Self = Self::new(2_959.955_381_693_075);
    /// The note Gb7.
    pub const GB7: Self = Self::FS7;

    /// The note G7.
    pub const G7: Self = Self::new(3_135.963_487_853_994);

    /// The note G#7.
    pub const GS7: Self = Self::new(3_322.437_580_639_561);
    /// The note Ab7.
    pub const AB7: Self = Self::GS7;

    /// The note A7.
    pub const A7: Self = Self::new(3520.0);

    /// The note A#7.
    pub const AS7: Self = Self::new(3_729.310_092_144_719_4);
    /// The note Bb7.
    pub const BB7: Self = Self::AS7;

    /// The note B7.
    pub const B7: Self = Self::new(3_951.066_410_048_993);
    /// The note Cb8.
    pub const CB8: Self = Self::B7;

    // OCTAVE 8

    /// The note B#7.
    pub const BS7: Self = Self::new(4_186.009_044_809_578);
    /// The note C8.
    pub const C8: Self = Self::BS7;

    /// The note C#8.
    pub const CS8: Self = Self::new(4_434.922_095_629_953);
    /// The note Db8.
    pub const DB8: Self = Self::CS8;

    /// The note D8.
    pub const D8: Self = Self::new(4_698.636_286_678_521);

    /// The note D#8.
    pub const DS8: Self = Self::new(4_978.031_739_553_295);
    /// The note Eb8.
    pub const EB8: Self = Self::DS8;

    /// The note E8.
    pub const E8: Self = Self::new(5_274.040_910_605_919);
    /// The note Fb8.
    pub const FB8: Self = Self::DS8;

    /// The note F8.
    pub const F8: Self = Self::new(5_587.651_702_928_062);
    /// The note E#8.
    pub const ES8: Self = Self::E8;

    /// The note F#8.
    pub const FS8: Self = Self::new(5_919.910_763_386_15);
    /// The note Gb8.
    pub const GB8: Self = Self::FS8;

    /// The note G8.
    pub const G8: Self = Self::new(6_271.926_975_707_988);

    /// The note G#8.
    pub const GS8: Self = Self::new(6_644.875_161_279_122);
    /// The note Ab8.
    pub const AB8: Self = Self::GS8;

    /// The note A8.
    pub const A8: Self = Self::new(7040.0);

    /// The note A#8.
    pub const AS8: Self = Self::new(7_458.620_184_289_439);
    /// The note Bb8.
    pub const BB8: Self = Self::AS8;

    /// The note B8.
    pub const B8: Self = Self::new(7_902.132_820_097_986);
    /// The note Cb9.
    pub const CB9: Self = Self::B8;

    // OCTAVE 9

    /// The note B#8.
    pub const BS8: Self = Self::new(8_372.018_089_619_156);
    /// The note C9.
    pub const C9: Self = Self::BS8;

    /// The note C#9.
    pub const CS9: Self = Self::new(8_869.844_191_259_906);
    /// The note Db9.
    pub const DB9: Self = Self::CS9;

    /// The note D9.
    pub const D9: Self = Self::new(9_397.272_573_357_042);

    /// The note D#9.
    pub const DS9: Self = Self::new(9_956.063_479_106_59);
    /// The note Eb9.
    pub const EB9: Self = Self::DS9;

    /// The note E9.
    pub const E9: Self = Self::new(10_548.081_821_211_837);
    /// The note Fb9.
    pub const FB9: Self = Self::DS9;

    /// The note F9.
    pub const F9: Self = Self::new(11_175.303_405_856_124);
    /// The note E#9.
    pub const ES9: Self = Self::E9;

    /// The note F#9.
    pub const FS9: Self = Self::new(11_839.821_526_772_3);
    /// The note Gb9.
    pub const GB9: Self = Self::FS9;

    /// The note G9.
    pub const G9: Self = Self::new(12_543.853_951_415_977);

    /// The note G#9.
    pub const GS9: Self = Self::new(13_289.750_322_558_244);
    /// The note Ab9.
    pub const AB9: Self = Self::GS9;

    /// The note A9.
    pub const A9: Self = Self::new(14080.0);

    /// The note A#9.
    pub const AS9: Self = Self::new(14_917.240_368_578_878);
    /// The note Bb9.
    pub const BB9: Self = Self::AS9;

    /// The note B9.
    pub const B9: Self = Self::new(15_804.265_640_195_972);
    /// The note Cb10.
    pub const CB10: Self = Self::B9;

    // OCTAVE 10

    /// The note B#9.
    pub const BS9: Self = Self::new(16_744.036_179_238_312);
    /// The note C10.
    pub const C10: Self = Self::BS9;

    /// The note C#10.
    pub const CS10: Self = Self::new(17_739.688_382_519_813);
    /// The note Db10.
    pub const DB10: Self = Self::CS10;

    /// The note D10.
    pub const D10: Self = Self::new(18_794.545_146_714_085);

    /// The note D#10.
    pub const DS10: Self = Self::new(19_912.126_958_213_18);
    /// The note Eb10.
    pub const EB10: Self = Self::DS10;

    /// The note E10.
    pub const E10: Self = Self::new(21_096.163_642_423_675);
    /// The note Fb10.
    pub const FB10: Self = Self::DS10;

    /// The note F10.
    pub const F10: Self = Self::new(22_350.606_811_712_25);
    /// The note E#10.
    pub const ES10: Self = Self::E10;

    /// The note F#10.
    pub const FS10: Self = Self::new(23_679.643_053_544_6);
    /// The note Gb10.
    pub const GB10: Self = Self::FS10;

    /// The note G10.
    pub const G10: Self = Self::new(25_087.707_902_831_953);

    /// The note G#10.
    pub const GS10: Self = Self::new(26_579.500_645_116_488);
    /// The note Ab10.
    pub const AB10: Self = Self::GS10;

    /// The note A10.
    pub const A10: Self = Self::new(28160.0);

    /// The note A#10.
    pub const AS10: Self = Self::new(29_834.480_737_157_755);
    /// The note Bb10.
    pub const BB10: Self = Self::AS10;

    /// The note B10.
    pub const B10: Self = Self::new(31_608.531_280_391_944);
    /// The note Cb11.
    pub const CB11: Self = Self::B10;
}
