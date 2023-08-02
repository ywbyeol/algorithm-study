const f = (n, m = {}) => {
  if (n <= 1n) return BigInt(n);
  if (m[n]) return m[n];
  return (m[n] = f(n - 1n, m) + f(n - 2n, m));
};
console.log(f(BigInt(`${require('fs').readFileSync(0)}`)).toString());