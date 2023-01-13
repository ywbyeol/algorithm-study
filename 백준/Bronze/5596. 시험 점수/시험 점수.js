const fs = require('fs');
const [s, t] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let S = 0;
let T = 0;
s.split(' ')
  .map(Number)
  .forEach((el) => {
    S += el;
  });
t.split(' ')
  .map(Number)
  .forEach((el) => {
    T += el;
  });
console.log(S >= T ? S : T);