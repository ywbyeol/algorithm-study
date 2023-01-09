const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let name = [];
let diff = [];
for (let i = 0; i < N; i++) {
  const inputArr = input[i].split(' ');
  name.push(String(inputArr[0]));
  diff.push(Number(inputArr[1]));
}
console.log(name[diff.indexOf(Math.min(...diff))]);