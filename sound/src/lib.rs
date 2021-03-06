//! synth test

extern crate rand; // for noise generation

mod pink;
mod white;

pub use pink::PnkGenerator;
pub use white::WhtGenerator;

/// An audio wave generator.
/// `(time_step, volume, time_passed)`
pub struct Generator(f64, f64, f64);

impl Generator {
	/// Create a new generator with default pitch[hz] and volume[0-1].
	#[inline(always)] pub fn new(hz: f64, volume: f64) -> Self {
		debug_assert!(volume <= 1.0 && volume >= 0.0);
		Generator(hz / 48_000.0, volume, 0.0)
	}

	/// Set pitch.  Takes in Hertz.
	#[inline(always)] pub fn pitch(&mut self, hz: f64) {
		self.0 = hz / 48_000.0;
	}

	/// Set volume.  Range 0 to 1
	#[inline(always)] pub fn volume(&mut self, volume: f64) {
		debug_assert!(volume <= 1.0 && volume >= 0.0);
		self.1 = volume;
	}

	/// Generate a sound.
	#[inline(always)]
	pub fn gen(&mut self, generator: &mut FnMut(f64) -> f64) -> i16 {
		// Generate the sample
		let sample = convert(generator(self.2), self.1);

		// Advance time_passed by time_step.
		self.2 += self.0;
		if self.2 >= 1.0 {
			self.2 -= 1.0;
		}

		// Return the sample
		sample
	}
}

/// Convert an f32 sample and volume to an i16 sample.
#[inline(always)] fn convert(sample: f64, volume: f64) -> i16 {
	(sample * (::std::i16::MAX as f64) * volume) as i16
}

/// A Saw wave
#[inline(always)] pub fn saw(x: f64) -> f64 {
	x * 2.0 - 1.0
}

/// A Sine wave
#[inline(always)] pub fn sin(x: f64) -> f64 {
	(x * (::std::f64::consts::PI * 2.0)).sin()
}

/// A triangle wave
#[inline(always)] pub fn tri(x: f64) -> f64 {
	(x * 2.0 - 1.0).abs() * 2.0 - 1.0
}

/// A square wave
#[inline(always)] pub fn sqr(x: f64) -> f64 {
	(x * 2.0 - 1.0).signum()
}

/// White noise
#[inline(always)] pub fn wht(white: &mut WhtGenerator) -> f64 {
	white.gen()
}

/// Pink noise
#[inline(always)] pub fn pnk(pink: &mut PnkGenerator) -> f64 {
	pink.gen()
}

/// Mix sound waves together.  (Add soundwaves together, then divide by 2)
#[inline(always)] pub fn mix(a: &[f64]) -> f64 {
	add(a) / (a.len() as f64)
}

/// Add sound waves together.  (Add sound waves ontop of eachother), may
/// introduce clipping.
#[inline(always)] pub fn add(a: &[f64]) -> f64 {
	let mut v = a[0];
	for i in a.iter().skip(1) {
		v += i;
	}
	v
}

/// Multiply sound waves together.
#[inline(always)] pub fn mul(a: &[f64]) -> f64 {
	let mut v = a[0];
	for i in a.iter().skip(1) {
		v *= i;
	}
	v
}

/// Negate sound wave (-x).
#[inline(always)] pub fn neg(x: f64) -> f64 {
	-x
}

/// Distort sound wave with hard clipping.  Volume should be more than 1 to have
/// any effect.
#[inline(always)] pub fn hrd(a: f64, volume: f64) -> f64 {
	(a * volume).min(1.0).max(-1.0)
}

/// Distort sound wave with soft clipping.  Volume should be more than 1 to have
/// any effect.
#[inline(always)] pub fn sft(a: f64, volume: f64) -> f64 {
	let max = (1.0 / (1.0 + (-volume).exp()) ) * 2.0 - 1.0;

	((1.0 / (1.0 + (a * -volume).exp()) ) * 2.0 - 1.0) / max
}

/// Square root of sound wave.
#[inline(always)] pub fn srt(a: f64) -> f64 {
	a.sqrt()
}

/// Cube root of sound wave.
#[inline(always)] pub fn crt(a: f64) -> f64 {
	a.cbrt()
}

/// Arcsine of a sound wave.
#[inline(always)] pub fn asn(a: f64) -> f64 {
	a.asin() * 2.0 / ::std::f64::consts::PI
}

/// Arcosine of a sound wave.
#[inline(always)] pub fn acs(a: f64) -> f64 {
	(a.acos() / ::std::f64::consts::PI) * 2.0 - 1.0
}

/// Arctangent of a sound wave.
#[inline(always)] pub fn atn(a: f64) -> f64 {
	a.atan() * 2.0 / ::std::f64::consts::PI
}

/// Signum of sound wave (-1 or 1)
#[inline(always)] pub fn sgn(a: f64) -> f64 {
	a.signum()
}

/// 8-bit conversion of sound wave.
#[inline(always)] pub fn bit(a: f64) -> f64 {
	/*-1 to 1, 0 to 1, 0 to 255, 0 to 1, 0 to 2, -1 to 1 */
	(((a * 0.5 + 0.5) * 255.0).round() / 255.0) * 2.0 - 1.0
}
