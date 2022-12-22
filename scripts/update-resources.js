const mjml2html = require('mjml');
const pretty = require('pretty');
const fsp = require('fs/promises');

const COMPARE_DIRECTORY = 'resources/compare/success';
const TEMPLATE_DIRECTORY = 'resources/template';

const iterate = (list, callback) =>
  list.reduce((res, item) => res.then(() => callback(item)), Promise.resolve());

const cleanupIds = (content) => {
  console.log(`⌛️ doing some cleanup on ids`);
  content = content
    .replace(/for="([a-zA-Z0-9]+)"/g, 'for="aaaaaaaa"')
    .replace(/id="([a-zA-Z0-9]+)"/g, 'id="aaaaaaaa"');
  let matches;
  while (matches = /mj-carousel-([a-zA-Z0-9]{12})-([a-zA-Z0-9\-]+)/g.exec(content)) {
    const carouselId = matches[1];
    const extension = matches[2];
    content = content.replaceAll(`mj-carousel-${carouselId}-${extension}`, `mj-carousel-aaaaaaaa-${extension}`);
    content = content.replaceAll(`mj-carousel-radio-${carouselId}`, `mj-carousel-radio-aaaaaaaa`);
  }
  return content;
};

const cleanup = (content) => {
  console.log(`⌛️ doing some more cleanup`);
  return content
    // empty style css blocks
    .replace(/<style\s+type="text\/css">\s*<\/style>/gim, '')
    .replace(/style=""/gim, '')
    // empty div blocks
    .replace(/<div\s*>\s*<\/div>/gim, '')
    .replace(/\s{2,}/gim, '\n')
    // percentages that are rounded in rust
    .replace(/33\.333333333333336/gm, '33.333332')
    .replace(/33-333333333333336/gm, '33-333332')
    ;
};

const handleFile = (dir, fname) => {
  console.log(`👉 starting ${dir}/${fname}`);
  return fsp.readFile(`${dir}/${fname}.mjml`, { encoding: 'utf8' })
    .then((content) => {
      console.log(`⌛️ converting ${dir}/${fname}.mjml`)
      return mjml2html(content).html;
    })
    .then(pretty)
    .then(cleanupIds)
    .then(cleanup)
    .then(pretty)
    .then((output) => {
      console.log(`⌛️ writing ${dir}/${fname}.html file`);
      return fsp.writeFile(`${dir}/${fname}.html`, output, { encoding: 'utf8' });
    })
    .then(() => console.log(`🎉 done with ${dir}/${fname}`))
    .catch((err) => console.error(`⛔️ error with ${fname}`, err));
}
const handleDirectory = (dir) => {
  console.log(`👉 starting with ${dir}`);
  return fsp.readdir(dir)
    .then((files) => files
      .filter((fname) => fname.endsWith('.mjml'))
      .map((fname) => fname.substring(0, fname.length - 5)))
    .then((files) => iterate(files, (fname) => handleFile(dir, fname)))
    .then(() => console.log(`🎉 done with ${dir}`));
};

iterate([COMPARE_DIRECTORY, TEMPLATE_DIRECTORY], handleDirectory)
  .then(() => console.log('🎉 All done'));
