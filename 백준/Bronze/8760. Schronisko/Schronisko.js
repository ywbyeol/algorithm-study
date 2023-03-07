process.stdin.on('data', i =>
  console.log(
    i
      .toString()
      .trim()
      .split('\n')
      .splice(1)
      .map(j => {
        const [W, K] = j.split(' ').map(Number);
        return Math.floor((W * K) / 2);
      })
      .join('\n'),
  ),
);