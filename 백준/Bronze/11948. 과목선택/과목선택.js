const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let arr1 = input.slice(0, 4).sort((a, b) => b - a);
let arr2 = input.slice(4).sort((a, b) => b - a);
console.log(arr1[0] + arr1[1] + arr1[2] + arr2[0]);