const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
input.sort((a, b) => a - b);
console.log(`${input[0]} ${input[1]} ${input[2]}`);