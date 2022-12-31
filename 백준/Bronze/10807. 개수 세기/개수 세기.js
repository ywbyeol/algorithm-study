const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
const inputArr = input[1].split(' ');
let count = 0;
for (let i = 0; i < input[0]; i++) {
  if (inputArr[i] === input[2]) {
    count++;
  }
}
console.log(count);