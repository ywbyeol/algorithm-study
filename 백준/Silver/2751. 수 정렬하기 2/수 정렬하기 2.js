  const fs = require('fs');
  const input = fs
    .readFileSync('/dev/stdin')
    .toString()
    .trim()
    .split('\n')
    .map(BigInt);
  const input_copy = [...input];
  input_copy.shift();
  input_copy.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
  const input_sorted = input_copy.join('\n');
  console.log(input_sorted);