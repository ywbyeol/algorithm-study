const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let num = Number(input);
let sum;
let i = 0;
while (true) {
  sum = Math.floor(num / 10) + (num % 10);
  num = Number(String(num % 10) + String(sum % 10));
  i++;
  if (num == input) {
    console.log(i);
    return;
  }
}