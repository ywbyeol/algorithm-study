const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let fibo = [0, 1];
if (input < 2) {
  console.log(input);
  return;
}
for (let i = 0; i < input; i++) {
  fibo.push(BigInt(fibo[i]) + BigInt(fibo[i + 1]));
  if (fibo[input] !== undefined) {
    console.log(fibo[input].toString());
    return;
  }
}