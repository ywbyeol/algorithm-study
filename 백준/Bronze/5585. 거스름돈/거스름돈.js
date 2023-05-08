let n = 1000 - +`${require('fs').readFileSync(0)}`.trim();
const arr = [500, 100, 50, 10, 5, 1];
let count = 0;
for (let i = 0; i < arr.length; i++) {
  count += Math.floor(n / arr[i]);
  n %= arr[i];
}
console.log(count);