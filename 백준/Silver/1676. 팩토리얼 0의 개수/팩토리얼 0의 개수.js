let n = +`${require('fs').readFileSync('/dev/stdin')}`.trim();
let ans = 0;
while (n > 0) {
  n = Math.floor(n / 5);
  ans += n;
}
console.log(ans);