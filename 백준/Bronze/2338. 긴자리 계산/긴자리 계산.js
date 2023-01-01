const fs = require('fs');
const [A, B] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(BigInt);
console.log(`${A + B}\n${A - B}\n${A * B}`);