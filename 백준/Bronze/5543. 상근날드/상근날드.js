const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
const burger = input.slice(0, 3).sort((a, b) => a - b);
const drink = input.slice(3, 6).sort((a, b) => a - b);
console.log(burger[0] + drink[0] - 50);