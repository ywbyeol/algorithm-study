const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
for (let i = 1; i <= input; i++) {
  console.log(`${' '.repeat(input - i)}${'*'.repeat(i)}`);
}