#import "plugin.typ": comet-plugin

#let fft-impl(
  values, direction: "forward"
) = {
  values = values.map(x => {
    if type(x) in (int, float) { return (float(x), 0.) }
    assert(
      type(x) == array and x.len() == 2,
      message: "A complex number can consist of one or two floats, got " + repr(x)
    )
    x.map(float)
  })

  let fft = if direction == "forward" { 
    comet-plugin.fft 
  } else { 
    comet-plugin.ifft 
  }

  cbor(fft(cbor.encode(values)))
}


/// Computes the discrete Fourier transform (DFT). 
/// 
/// Returns an array of complex (i.e., real/imaginary pairs of floats) values. 
#let fft(

  /// An array of real (`float`) or complex (real/imaginary pairs of `float`) 
  /// values. 
  /// -> array
  values

) = fft-impl(values, direction: "forward")


/// Computes the inverse discrete Fourier transform (DFT). 
/// 
/// Returns an array of complex (i.e., real/imaginary pairs of floats) values. 
#let ifft(

  /// An array of real (`float`) or complex (real/imaginary pairs of `float`) 
  /// values. 
  /// -> array
  values

) = fft-impl(values, direction: "inverse")
