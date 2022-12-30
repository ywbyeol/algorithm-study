const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
for (let i = 0; i < input.length; i++) {
  const num = input[i].split(' ');
  str += Number(num[0]) + Number(num[1]) + '\n';
}
console.log(str.trim().slice(0, -1));