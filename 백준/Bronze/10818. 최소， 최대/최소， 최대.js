const fs = require('fs');
const [N, input] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
const inputArr = input.split(' ').map(Number);
console.log(`${Math.min(...inputArr)} ${Math.max(...inputArr)}`);