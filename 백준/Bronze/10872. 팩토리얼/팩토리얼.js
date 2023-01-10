const fs = require('fs');
const N = fs.readFileSync('/dev/stdin').toString().trim();
let fac = 1;
for (let i = 1; i <= N; i++) {
  fac *= i;
}
console.log(fac);