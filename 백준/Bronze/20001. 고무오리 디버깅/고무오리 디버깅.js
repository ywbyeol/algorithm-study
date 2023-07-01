const i = `${require('fs').readFileSync(0)}`.trim().split('\n').slice(1, -1);
let cnt = 0;
for (const str of i) cnt += str.trim() === '문제' ? 1 : cnt < 1 ? 2 : -1;
console.log(cnt < 1 ? '고무오리야 사랑해' : '힝구');