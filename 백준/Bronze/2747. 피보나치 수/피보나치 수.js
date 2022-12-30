const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let fibo = [0, 1];
for (let i = 0; i < input; i++) {
  fibo.push(Number(fibo[i]) + Number(fibo[i + 1]));
  if (fibo[input] !== undefined) {
    console.log(fibo[input]);
    return;
  }
}