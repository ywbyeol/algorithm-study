console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split('\n')
    .slice(1)
    .reduce((a, b) => {
      return +a * +b;
    }),
);