const fac = N => {
  let res = 1n;
  for (let i = 2n; i <= BigInt(N); i++) {
    res *= i;
  }
  return res;
};
const val = fac(+`${require('fs').readFileSync('/dev/stdin')}`.trim())
  .toString()
  .split(/[^0]+/g)
  .filter(x => x !== '')
  .at(-1)?.length;
console.log(val === undefined ? 0 : val);