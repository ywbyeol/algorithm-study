let [N, i] = [`${require('fs').readFileSync(0)}`.trim(), 0];
do {
  let s = `${++i}`;
  while (s.length > 0 && N.length > 0) {
    if (s[0] === N[0]) N = N.slice(1);
    s = s.slice(1);
  }
} while (N !== '');
console.log(i);