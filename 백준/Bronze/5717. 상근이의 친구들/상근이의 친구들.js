console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split('\n')
    .slice(0, -1)
    .map(x => eval(x.replace(' ', '+')))
    .join('\n')
);