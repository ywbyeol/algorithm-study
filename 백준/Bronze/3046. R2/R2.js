const fs = require('fs');
const [R1, S] = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
console.log(2 * S - R1);
