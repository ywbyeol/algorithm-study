const fs = require('fs');
const [A, B, C] = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
let prize = 0;
const a = parseInt(A);
const b = parseInt(B);
const c = parseInt(C);
if (a === b && b === c) {
  prize += 10000 + a * 1000;
  console.log(prize);
  return;
}
if (a === b || b === c) {
  prize += 1000 + b * 100;
  console.log(prize);
  return;
} else if (c === a) {
  prize += 1000 + a * 100;
  console.log(prize);
  return;
}
prize += Math.max(a, b, c) * 100;
console.log(prize);