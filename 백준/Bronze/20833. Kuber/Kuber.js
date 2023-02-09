const fs = require('fs');
const N = Number(fs.readFileSync('/dev/stdin').toString().trim());
let answer = 0;
for (let i = 1; i <= N; i++) {
  answer += i ** 3;
}
console.log(answer);