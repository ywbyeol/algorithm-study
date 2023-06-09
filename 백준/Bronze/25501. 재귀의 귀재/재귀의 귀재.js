const [T, ...S] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g);
let cnt = 0;
const recursion = (s, l, r) => {
  cnt++;
  if (l >= r) return 1;
  if (s[l] !== s[r]) return 0;
  return recursion(s, l + 1, r - 1);
};
const isPalindrome = s => recursion(s, 0, s.length - 1);
const ans = [];
for (let i = 0; i < +T; i++, cnt = 0) ans.push(`${isPalindrome(S[i])} ${cnt}`);
console.log(ans.join('\n'));