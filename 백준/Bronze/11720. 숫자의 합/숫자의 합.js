const fs = require('fs');
const [N, input] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
sum = 0;
for (let i = 0; i < N; i++) {
  sum += Number(input[i]);
}
console.log(sum);
