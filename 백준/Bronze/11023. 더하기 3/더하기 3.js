const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
let sum = 0;
for (const i of input) {
  sum += parseInt(i);
}
console.log(sum);