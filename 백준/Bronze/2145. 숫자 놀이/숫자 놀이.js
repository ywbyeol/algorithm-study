console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(0, -1)
    .map(x => {
      let num = +x;
      do {
        num = [...`${num}`].reduce((a, b) => a + +b, 0);
      } while (`${num}`.length > 1);
      return num;
    })
    .join('\n'),
);