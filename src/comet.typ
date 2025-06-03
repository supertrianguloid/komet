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




#let fft(
  data
) = {

  let input = cbor.encode(
    data.map(x => {
      if type(x) in (int, float) { (x, 0) }
      else if type(x) == array {
        assert(x.len() == 2)
        x
      } else {
        assert(false )
      }
    })
  )

  cbor(p.fft(input))
}

#fft((3,3,5))