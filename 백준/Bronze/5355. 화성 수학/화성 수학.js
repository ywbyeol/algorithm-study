const T = `${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .slice(1)
  .map(v => {
    const [ini, ...ope] = v.trim().split(' ');
    let num = +ini;
    for (const i of ope) num += i === '@' ? num * 2 : i === '%' ? 5 : -7;
    return num.toFixed(2);
  });
console.log(T.join('\n'));