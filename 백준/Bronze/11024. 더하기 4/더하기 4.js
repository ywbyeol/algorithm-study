const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < N; i++) {
  const arr = input[i].split(' ').map(Number);
  let sum = 0;
  for (let j = 0; j < arr.length; j++) {
    sum += arr[j];
  }
  str += `${sum}\n`;
}
console.log(str.trim());