console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .match(/.{1,10}/g)
    .join('\n'),
);