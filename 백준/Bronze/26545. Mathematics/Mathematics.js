process.stdin.on('data', i =>
  console.log(
    i
      .toString()
      .trim()
      .split('\n')
      .slice(1)
      .reduce((a, b) => a + Number(b), 0),
  ),
);