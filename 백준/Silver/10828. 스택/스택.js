const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
const stack = {
  value: [],
  index: 0,
  result: '',
  answer: '',
  push: function (X) {
    this.value[stack.index++] = X;
  },
  pop: function () {
    if (this.index) {
      this.index -= 1;
      this.answer += this.value.splice(-1)[0] + '\n';
    } else {
      this.answer += '-1\n';
    }
  },
  size: function () {
    this.result = this.index;
    this.answer += this.result + '\n';
  },
  empty: function () {
    this.result = this.index ? 0 : 1;
    this.answer += this.result + '\n';
  },
  top: function () {
    this.result = this.index ? this.value[this.index - 1] : -1;
    this.answer += this.result + '\n';
  },
  run: function () {
    for (let i = 0; i < N; i++) {
      if (!isNaN(input[i].at(-1))) {
        this.push(input[i].substring('5'));
        continue;
      }
      if (input[i] === 'pop') {
        this.pop();
        continue;
      }
      if (input[i] === 'size') {
        this.size();
        continue;
      }
      if (input[i] === 'empty') {
        this.empty();
        continue;
      }
      if (input[i] === 'top') {
        this.top();
      }
    }
    console.log(this.answer.trim());
  },
};

stack.run();