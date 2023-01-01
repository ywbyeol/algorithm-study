const fs = require('fs');
const [A, B, C] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
B >= C ? console.log(-1) : console.log(Math.floor(A / (C - B)) + 1);