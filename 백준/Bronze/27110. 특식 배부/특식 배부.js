const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().split('\n');
const N = Number(input[0]);
const [A, B, C] = input[1].split(' ').map(Number);
console.log((N > A ? A : N) + (N > B ? B : N) + (N > C ? C : N));