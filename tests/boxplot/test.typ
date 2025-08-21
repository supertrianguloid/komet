#import "/src/komet.typ": boxplot

#assert.eq(boxplot((1, 2, 3, 4, -2, 100)), (
  mean: 18.0,
  median: 2.5,
  q1: 1.25,
  q3: 3.75,
  min: -2.,
  max: 100.,
  whisker-low: -2.,
  whisker-high: 4.,
  outliers: (100.,),
))

#assert.eq(boxplot((1, 2, 3, 4)), (
  mean: 2.5,
  median: 2.5,
  q1: 1.75,
  q3: 3.25,
  min: 1.,
  max: 4.,
  whisker-low: 1.,
  whisker-high: 4.,
  outliers: (),
))
