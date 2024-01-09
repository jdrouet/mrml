import { render as renderBundler } from './target-bundler';
import { render as renderWeb } from './target-web';

const input = document.getElementById('input');
const output = document.getElementById('output');

function handlerResult(res) {
  if (res.type === 'success') {
    output.value = res.content;
  } else {
    output.value = JSON.stringify(res, null, 2);
  }
}

document.getElementById('build-bundler').addEventListener('click', function () {
  renderBundler(input.value).then(handlerResult);
});

document.getElementById('build-web').addEventListener('click', function () {
  renderWeb(input.value).then(handlerResult);
});
