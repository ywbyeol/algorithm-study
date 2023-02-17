process.stdin.on('data', i => {
  console.log(
    i
      .toString()
      .trim()
      .split('\n')
      .map(Number)
      .reduce((a, b) => b - a),
  );
});