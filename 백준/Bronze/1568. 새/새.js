let i = +`${require('fs').readFileSync(0)}`.trim();
let cnt = 1;
let sec = 0;
while (i > 0) {
  if (i < cnt) cnt = 1;
  i -= cnt;
  cnt++;
  sec++;
}
console.log(sec);