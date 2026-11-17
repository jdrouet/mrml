import subprocess,json,pathlib,time,os
p=pathlib.Path(__file__).resolve().parent
out=p/'results'; out.mkdir(exist_ok=True)
base=['docker','run','--rm','--cpuset-cpus=2','--cpus=1','--memory=1g','--network=none','-v',str(out)+':/out']
with (out/'runs.jsonl').open('w') as log:
 for repeat in range(3):
  for template in ['small','medium','large']:
   engines=['mjml','mrml'] if repeat%2==0 else ['mrml','mjml']
   for engine in engines:
    command=['node','bench.cjs'] if engine=='mjml' else ['mrml-bench']
    r=subprocess.run(base+[os.environ.get('BENCHMARK_IMAGE','mjml-mrml-bench:warm-render')]+command+['templates/'+template+'.mjml','/out/'+template+'-'+engine+'.html'],capture_output=True,text=True,check=True)
    data=json.loads(r.stdout);data['repeat']=repeat+1
    log.write(json.dumps(data)+'\n');log.flush();print(json.dumps(data),flush=True)
