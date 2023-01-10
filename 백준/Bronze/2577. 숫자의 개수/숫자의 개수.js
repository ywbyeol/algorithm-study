const fs = require('fs');
const [A, B, C] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
const prod = String(A * B * C);
let str = '';
for (let i = 0; i < 10; i++) {
  let sum = 0;
  for (let j = 0; j < prod.length; j++) {
    if (prod[j] == i) {
      sum++;
    }
  }
  str += `${sum}\n`;
}
console.log(str.trim());