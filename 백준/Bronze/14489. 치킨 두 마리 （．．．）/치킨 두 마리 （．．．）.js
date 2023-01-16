const fs = require('fs');
const [a, b] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
const arr = a.split(' ').map(Number);
const balance = arr[0] + arr[1];
const cost = 2 * Number(b);
console.log(balance >= cost ? balance - cost : balance);