const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let sum = 0;
for (let i = 2; i < Number(input[1]) + 2; i++) {
  const el = input[i].split(' ');
  sum += Number(el[0]) * Number(el[1]);
}
if (sum == input[0]) {
  console.log('Yes');
  return;
}
console.log('No');