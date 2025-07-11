#let p = plugin("../comet/target/wasm32-unknown-unknown/release/comet.wasm")


/// Computes a histogram by counting values that fall into a set of intervals,
/// the so-called bins. 
/// 
/// Returns a dictionary `(counts: array, edges: array)` of histogram counts 
/// and bin edges. 
/// 
/// -> dictionary
#let histogram(
  
  /// An array of float or integer values to compute a histogram from. 
  /// -> array
  data,

  /// A number of bins or an explicit array of bin edges. If $n+1$ bin edges
  /// $e_1,...,e_(n+1)$ are given, the data will be sorted into $n$ bins 
  /// $[e_i, e_(i+1))$ for $i=1,...,n-1$ and $[e_n,e_(n+1)]$. 
  /// -> int | array
  bins: 10,

) = {
  if type(bins) == array {
    bins = bins.map(float).sorted()
  }

  let input = cbor.encode((
    data.map(float),
    bins,
  ))

  cbor(p.histogram(input))
}





#let fft-impl(
  data, direction: "forward"
) = {
  data = data.map(x => {
    if type(x) in (int, float) { return (float(x), 0.) }
    assert(
      type(x) == array and x.len() == 2,
      message: "A complex number can consist of one or two floats, got " + repr(x)
    )
    x.map(float)
  })

  let fft = if direction == "forward" { p.fft } else { p.ifft }

  cbor(fft(cbor.encode(data)))
}


/// Computes the discrete Fourier transform (DFT). 
/// 
/// Returns an array of complex (i.e., real-imaginary pairs of floats) values. 
#let fft(

  /// An array of real (`float`) or complex (real-imaginary pairs of `float`) 
  /// values. 
  /// -> array
  data

) = fft-impl(data, direction: "forward")


/// Computes the inverse discrete Fourier transform (DFT). 
/// 
/// Returns an array of complex (i.e., real-imaginary pairs of floats) values. 
#let ifft(

  /// An array of real (`float`) or complex (real-imaginary pairs of `float`) 
  /// values. 
  /// -> array
  data

) = fft-impl(data, direction: "inverse")


#let contour(
  x, 
  y, 
  z, 
  levels
) = {
  
  
  if type(z) == function {
    z = y.map(y => x.map(x => z(x, y)))
  }

  if type(levels) in (int, float) {
    levels = (levels,)
  }

  let data = cbor.encode((
    x.map(float),
    y.map(float),
    z.flatten().map(float),
    levels.map(float)
  ))
  cbor(p.contour(data))
}

#contour((0, 1), (0, 1), ((-1,1,1,-1)), (0,))

// #let o = contour(
//     lq.linspace(-2, 2, num: 1000),
//     lq.linspace(-2, 2, num: 1000),
//     (x, y) => x * y,
//     (-2, -1.5, -1, -.5, 0, .5, 1, 1.5, 2)
//   )
// )
// #o
a a

#{
  let t = range(30)
  let a = t.map(t => calc.sin(t/10) + calc.sin(t/1))
  let f = fft(a)
  let len = f.len()
  let b = ifft(f)
  // let a = b.map(x => x.first())
  let mag = f.map(a => calc.norm(..a))
  import "@preview/lilaq:0.3.0" as lq


  lq.diagram(
    width: 8cm,
    yaxis: (position: left),
    lq.plot(t, a, mark: none),

    lq.yaxis(
      position: right,
      lq.plot(t, mag),
    )

  )
}

#ifft(fft(((1,2),2,3,4,5)))
#fft(((1,2),2,3,4,5))
