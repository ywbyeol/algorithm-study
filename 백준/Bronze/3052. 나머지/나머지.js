const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let rem = [];
let diff = [];
for (let i = 0; i < input.length; i++) {
  rem.push(input[i] % 42);
}
for (let i = 0; i < rem.length; i++) {
  if (!diff.includes(rem[i])) {
    diff.push(rem[i]);
  }
}
console.log(diff.length);
