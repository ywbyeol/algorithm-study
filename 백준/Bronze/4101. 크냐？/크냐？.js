const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
input.pop();
for (let i = 0; i < input.length; i++) {
  const inputArr = input[i].split(' ').map(Number);
  str += inputArr[0] - inputArr[1] > 0 ? `Yes\n` : `No\n`;
}
console.log(str.trim());