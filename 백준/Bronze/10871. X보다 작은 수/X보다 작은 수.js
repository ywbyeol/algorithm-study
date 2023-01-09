const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
const [N, X] = input[0].split(' ').map(Number);
const inputArr = input[1].split(' ').map(Number);
let str = '';
for (let i = 0; i < N; i++) {
  if (inputArr[i] < X) {
    str += `${inputArr[i]} `;
  }
}
console.log(str.trim());