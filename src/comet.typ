#let p = plugin("../comet/target/wasm32-unknown-unknown/release/comet.wasm")



#let histogram(

  /// -> array
  data,

  /// -> int | array
  bins: 10,

) = {
  if type(bins) == array {
    bins = bins.map(float)
  }
  let input = cbor.encode((
    data.map(float),
    bins,
  ))

  cbor(p.histogram(input))
}

// #bin(range(1234), bins: (1,3,555,5556))


#let data = (8, 7, 9, 0, 4, 7, 2, 9, 0, 8, 3, 9, 2, 10, 10, 6, 9, 7, 6, 9) * 10000

#let bin2(data, bins: 5, lower: auto, upper: auto) = {
  let nbins = bins
  if lower == auto { lower = calc.min(..data) }
  if upper == auto { upper = calc.max(..data) }
  let stepsize = (upper - lower) / nbins
  let boundaries = ()
  for i in range(nbins + 1) {
    boundaries.push(lower + i * stepsize)
  }
  let freq = (underflow: 0, freq: range(nbins).map(x => 0), overflow: 0)
  let windows = boundaries.windows(2)
  let binned = data.fold(
    freq,
    (acc, val) => {
      let bin = int((val - lower) / (upper - lower) * nbins)
      if bin < 0 { acc.underflow += 1 } else if bin < nbins { acc.freq.at(bin) += 1 } else { acc.overflow += 1 }
      acc
    },
  )
  (
    lower: lower,
    upper: upper,
    stepsize: stepsize,
    boundaries: boundaries,
    windows: windows,
    centres: windows.map(((x, y)) => (x + y) / 2),
    binned: binned,
  )
  // return binned.freq
}

#histogram(range(5), bins: 3)
// #bin(range(100000), bins: 100)
// #bin(data, bins: 5)
