const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
input.sort((a, b) => {
  return b - a;
});
console.log(input[1]);