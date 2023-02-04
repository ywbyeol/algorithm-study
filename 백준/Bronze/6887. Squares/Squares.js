const fs = require('fs');
const input = Number(fs.readFileSync('/dev/stdin').toString());
console.log(
  `The largest square has side length ${Math.floor(Math.sqrt(input))}.`,
);