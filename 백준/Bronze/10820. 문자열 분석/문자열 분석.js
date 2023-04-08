const input = `${require('fs').readFileSync('/dev/stdin')}`
  .split('\n')
  .filter(Boolean);
console.log(
  input
    .map(
      x =>
        `${(x.match(/[a-z]/g) || []).length} ${
          (x.match(/[A-Z]/g) || []).length
        } ${(x.match(/[0-9]/g) || []).length} ${(x.match(/ /g) || []).length}`,
    )
    .join('\n'),
);