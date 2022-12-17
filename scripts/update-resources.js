const mjml2html = require('mjml');
const fs = require('fs');
const pretty = require('pretty');
const fsp = require('fs/promises');

const COMPARE_DIRECTORY = 'resources/compare/success';
const TEMPLATE_DIRECTORY = 'resources/template';

const cleanupIds = (content) => {
  let matches;
  while (matches = /for="([a-zA-Z0-9]+)"/g.exec(content)) {
    content = content.replaceAll(matches[1], 'aaaaaaaa');
  }
  while (matches = /mj-carousel-([a-zA-Z0-9]{12})-([a-zA-Z0-9\-]+)/g.exec(content)) {
    const carouselId = matches[1];
    const extension = matches[2];
    content = content.replaceAll(`mj-carousel-${carouselId}-${extension}`, `mj-carousel-aaaaaaaa-${extension}`);
    content = content.replaceAll(`mj-carousel-radio-${carouselId}`, `mj-carousel-radio-aaaaaaaa`);
  }
  return content;
};

const cleanup = (content) =>
  content
    // empty style css blocks
    .replace(/<style\s+type="text\/css">\s*<\/style>/gim, '')
    .replace(/style=""/gim, '')
    // empty div blocks
    .replace(/<div\s*>\s*<\/div>/gim, '')
    .replace(/\s{2,}/gim, '\n')
    // percentages that are rounded in rust
    .replace(/33\.333333333333336/gm, '33.333332')
    .replace(/33-333333333333336/gm, '33-333332')
    .replace(/cell-padding="(.*)"/gmi, 'cellpadding="$1"')
    .replace(/cell-spacing="(.*)"/gmi, 'cellspacing="$1"')
  ;

const handleFile = (dir) => (fname) =>
  fsp.readFile(`${dir}/${fname}.mjml`, { encoding: 'utf8' })
    .then((content) => mjml2html(content).html)
    .then(pretty)
    .then(cleanupIds)
    .then(cleanup)
    .then(pretty)
    .then((output) => fsp.writeFile(`${dir}/${fname}.html`, output, { encoding: 'utf8' }))
    .catch((err) => console.error(`error with ${fname}`, err));

const handleDirectory = (dir) =>
  Promise.all(fs.readdirSync(dir)
    .filter((fname) => fname.endsWith('.mjml'))
    .map((fname) => fname.substring(0, fname.length - 5))
    .map(handleFile(dir)))
    .then(() => console.log('done !'));

// Promise.all(fs.readdirSync(COMPARE_DIRECTORY)
//   .filter((fname) => fname.endsWith('.mjml'))
//   .map((fname) => fname.substring(0, fname.length - 5))
//   .map(handleFile(COMPARE_DIRECTORY)))
//   .then(() => console.log('done !'));

// Promise.all(fs.readdirSync(TEMPLATE_DIRECTORY)
//   .filter((fname) => fname.endsWith('.mjml'))
//   .map((fname) => fname.substring(0, fname.length - 5))
//   .map(handleFile(TEMPLATE_DIRECTORY)))
//   .then(() => console.log('done !'));
