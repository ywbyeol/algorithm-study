console.log(
  require('fs')
    .readFileSync(0, 'utf8')
    .trim()
    .split('\n')
    .slice(1)
    .map(s => {
      const [lt, wt, le, we] = s.split(' ').map(BigInt);
      const T = lt * wt;
      const E = le * we;
      return E === T ? 'Tie' : E > T ? 'Eurecom' : 'TelecomParisTech';
    })
    .join('\n'),
);