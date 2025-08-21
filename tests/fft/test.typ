#import "/src/komet.typ": fft, ifft

#assert.eq(
  fft((1, 2, 3, 4)),
  ((10, 0), (-2, 2), (-2, 0), (-2, -2)),
)

#assert.eq(
  fft(((1, 2), (-3, -8), (3, 9), (4, -7))),
  ((5, -4), (-3, 0), (3, 26), (-1, -14)),
)

#let test-inverse(values) = {
  assert.eq(
    ifft(fft(values)),
    values,
  )
}

#test-inverse(((1, 0), (2, 9), (-233, -2), (4, 0)))
