const fs = require('fs');
const [LP, input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
const [L, P] = LP.split(' ').map(Number);
const entrant = L * P;
const inputArr = input.split(' ').map(Number);
let str = '';
for (let i = 0; i < inputArr.length; i++) {
  str += `${inputArr[i] - entrant} `;
}
console.log(str.trim());