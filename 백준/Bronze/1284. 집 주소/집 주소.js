console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split(/\s+/g)
    .slice(0, -1)
    .map(
      x =>
        x.length +
        1 +
        x
          .split('')
          .map(y => (+y === 0 ? 4 : +y === 1 ? 2 : 3))
          .reduce((a, b) => a + b, 0),
    )
    .join('\n'),
);