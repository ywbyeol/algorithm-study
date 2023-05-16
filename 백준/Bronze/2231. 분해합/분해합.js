const N = +`${require('fs').readFileSync(0)}`.trim();
let ans = 0;
for (let i = 1; i < N; i++) {
  const sum = Array.from(String(i), Number).reduce((a, b) => a + b, 0);
  if (i + sum === N) {
    ans = i;
    break;
  }
}
console.log(ans);