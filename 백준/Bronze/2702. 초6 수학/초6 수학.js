const i = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g).slice(1);
const gcd = (a, b) => {
  if (b === 0) return a;
  return gcd(b, a % b);
};
const ans = [];
for (let j = 0; j < i.length; j += 2) {
  const gcdVal = gcd(i[j], i[j + 1]);
  ans.push(`${(+i[j] * +i[j + 1]) / gcdVal} ${gcdVal}`);
}
console.log(ans.join('\n'));