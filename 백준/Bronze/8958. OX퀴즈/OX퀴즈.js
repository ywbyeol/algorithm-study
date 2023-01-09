const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(String);
let str = '';
for (let i = 0; i < N; i++) {
  const arr1 = input[i].split('X').filter(Boolean);
  let num = 0;
  for (let j = 0; j < arr1.length; j++) {
    let arr2 = arr1[j];
    let sum = 0;
    let count = 0;
    for (let k = 0; k < arr2.length; k++) {
      if (arr2[k] === 'O') {
        count++;
      } else {
        count = 0;
      }
      sum += count;
    }
    num += sum;
  }
  str += `${num}\n`;
}
console.log(str.trim());