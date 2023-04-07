let N = +`${require('fs').readFileSync('/dev/stdin')}`.trim();
let ans = 1;
for (let i = 2; i * i <= N; i++) {
  let sum = 1;
  while (N % i === 0) {
    N /= i;
    sum = sum * i + 1;
  }
  ans *= sum;
}
ans *= N > 1 ? 1 + N : 1;
console.log(ans * 5 - 24);