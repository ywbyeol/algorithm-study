console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split('')
    .filter(a => 'aiueo'.includes(a)).length,
);