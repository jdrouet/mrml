const fs=require('fs'), mjml=require('mjml');
const [file,out]=process.argv.slice(2);
async function main(){
const source=fs.readFileSync(file,'utf8');
async function render(){ const r=await mjml(source,{validationLevel:'skip',minify:false}); if(!r.html)throw Error('empty'); return r.html; }
fs.writeFileSync(out,await render());
if(process.env.CHECK_ONLY)process.exit(0);
const warm=Date.now()+Number(process.env.WARM_SECONDS||5)*1000; while(Date.now()<warm)await render();
const samples=[]; let bytes=0; const start=process.hrtime.bigint();
while(Number(process.hrtime.bigint()-start)/1e9<Number(process.env.BENCH_SECONDS||10)){const t=process.hrtime.bigint();const html=await render();bytes+=Buffer.byteLength(html);samples.push(Number(process.hrtime.bigint()-t)/1e6);}
const elapsed=Number(process.hrtime.bigint()-start)/1e9;samples.sort((a,b)=>a-b);
console.log(JSON.stringify({engine:'mjml',template:file,count:samples.length,seconds:elapsed,rps:samples.length/elapsed,p50_ms:samples[Math.floor(samples.length*.5)],p95_ms:samples[Math.floor(samples.length*.95)],bytes,maxrss_kb:process.resourceUsage().maxRSS}));

}
main().catch(e=>{console.error(e);process.exit(1)});
