const fs = require('fs');
const [A, B] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number)
  .sort((a, b) => b - a);
if (A === B) {
  console.log('0');
  return;
}
let str = '';
let N = Math.abs(A - B) - 1;
str += `${N}\n`;
for (let i = B + 1; i < A; i++) {
  str += `${i} `;
}
console.log(str.trim());