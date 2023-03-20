console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split('\n')
    .slice(1)
    .map(Number)
    .sort((a, b) => a - b)
    .join('\n'),
);