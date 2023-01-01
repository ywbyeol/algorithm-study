const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
const input_copy = [...input];
input_copy.shift();
input_copy.sort((a, b) => a - b);
const input_sorted = input_copy.join('\n');
console.log(input_sorted);