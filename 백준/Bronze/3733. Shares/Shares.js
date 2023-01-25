const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
for (let i = 0; i < input.length; i += 1) {
  const arr = input[i].split(' ').map(Number);
  str += `${Math.floor(arr[1] / (arr[0] + 1))}\n`;
}
console.log(str.trim());