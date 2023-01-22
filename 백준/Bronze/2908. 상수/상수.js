const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
const num1 = input[2] + input[1] + input[0];
const num2 = input[6] + input[5] + input[4];
console.log(num1 > num2 ? num1 : num2);