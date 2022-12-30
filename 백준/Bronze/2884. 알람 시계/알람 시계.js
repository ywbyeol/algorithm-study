const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
const num1 = parseInt(input[0]) * 60 + parseInt(input[1]) - 45;
let num2 = Math.floor(num1 / 60);
let num3 = num1 % 60;
if (num2 < 0) {
  num2 += 24;
}
if (num3 < 0) {
  num3 += 60;
}
console.log(`${num2} ${num3}`);
