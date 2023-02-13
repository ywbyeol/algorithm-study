process.stdin.on('data', i =>
  console.log(
    i
      .toString()
      .trim()
      .split(' ')
      .map(v => +v * 5)
      .reduce((a, b) => a + b, 0),
  ),
);