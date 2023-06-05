const ans = [];
const isStrange = num => {
  const sum = rad =>
    num
      .toString(rad)
      .split('')
      .map(v => parseInt(v, rad).toString(10))
      .reduce((a, b) => a + +b, 0);
  return sum(10) === sum(12) && sum(12) === sum(16);
};
for (let i = 2992; i < 10000; i++) if (isStrange(i)) ans.push(i);
console.log(ans.join('\n'));