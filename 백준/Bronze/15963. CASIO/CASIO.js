const fs = require('fs');
[N, M] = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
console.log(Number(N === M));