const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
if (input % 400 === 0 || (input % 4 === 0 && input % 100 !== 0)) {
  console.log(1);
  return;
}
console.log(0);