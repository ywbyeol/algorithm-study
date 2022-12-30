const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
for (let i = 1; i <= input[0]; i++) {
  const num = input[i].split(' ');
  str += Number(num[0]) + Number(num[1]) + '\n';
}
console.log(str.trim());