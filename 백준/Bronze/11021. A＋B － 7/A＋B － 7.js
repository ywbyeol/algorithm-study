const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
for (let i = 1; i < Number(input[0]) + 1; i++) {
  const el = input[i].split(' ');
  str += `Case #${i}: ${Number(el[0]) + Number(el[1])}
`;
}
console.log(str.trim());