console.log(
  `${require('fs').readFileSync('/dev/stdin')}`
    .split(/\s+/g)
    .slice(1)
    .filter(x => {
      return +x < 2
        ? false
        : Array.from(
            { length: Math.floor(Math.sqrt(+x)) - 1 },
            (_, i) => i + 2,
          ).every(i => +x % i !== 0);
    }).length,
);