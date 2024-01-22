const n = +`${require('fs').readFileSync(0)}`.trim();
let [l, r] = [0, n];
while (l <= r) {
  const m = Math.floor((l + r) / 2);
  if (BigInt(m) ** 2n < n) {
    l = m + 1;
  } else {
    r = m - 1;
  }
}
console.log(l);