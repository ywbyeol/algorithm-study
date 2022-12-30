const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let fibo = [0, 1];
if (input == 0) {
  console.log(0);
  return;
}
for (let i = 0; i < input; i++) {
  fibo.push(Number(fibo[i]) + Number(fibo[i + 1]));
  if (fibo[input] !== undefined) {
    console.log(fibo[input]);
    return;
  }
}