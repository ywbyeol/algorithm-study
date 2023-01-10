const fs = require('fs');
const [N, S] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let count2 = 0;
let countE = 0;
for (let i = 0; i < N; i++) {
  if (S[i] == 2) {
    count2++;
    continue;
  }
  countE++;
}
if (count2 === countE) {
  console.log('yee');
  return;
}
console.log(count2 > countE ? '2' : 'e');