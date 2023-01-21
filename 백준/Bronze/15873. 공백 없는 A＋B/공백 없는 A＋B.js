const fs = require('fs');
const num = fs.readFileSync('/dev/stdin').toString().trim();
let answer;
if (num.length === 4) {
  answer = 20;
} else if (num.length === 2) {
  answer = Number(num[0]) + Number(num[1]);
} else if (Number(num[1]) === 0) {
  answer = Number(num.slice(0, 2)) + Number(num.slice(2));
} else {
  answer = Number(num.slice(0, 1)) + Number(num.slice(1));
}
console.log(answer);