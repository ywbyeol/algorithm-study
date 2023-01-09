const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
const max = Math.max(...input);
console.log(`${max}\n${input.indexOf(max) + 1}`);