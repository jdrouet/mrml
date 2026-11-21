import json,pathlib,subprocess,os
p=pathlib.Path(__file__).resolve().parent
out=p/'results/resources';out.mkdir(parents=True,exist_ok=True)
with (out/'runs.jsonl').open('w') as log:
 for engine,template,cap,seconds in [(e,t,'1g',30) for t in ['small','medium','large'] for e in ['mjml','mrml']]+[(e,t,c,10) for e,c in [('mjml','256m'),('mrml','64m')] for t in ['small','large']]:
  cmd=['node','bench.cjs'] if engine=='mjml' else ['mrml-bench']
  r=subprocess.run(['docker','run','--rm','--cpuset-cpus=2','--cpus=1','--memory='+cap,'--memory-swap='+cap,'--pids-limit=64','--network=none','-e','BENCH_SECONDS='+str(seconds),'-v',str(out)+':/out',os.environ.get('BENCHMARK_IMAGE','mjml-mrml-bench:warm-render'),'resource-monitor']+cmd+['templates/'+template+'.mjml','/out/'+template+'-'+engine+'.html'],capture_output=True,text=True)
  resource=[l.removeprefix('RESOURCE ') for l in r.stderr.splitlines() if l.startswith('RESOURCE ')]
  data={'engine':engine,'template':template,'memory_limit':cap,'requested_seconds':seconds,'exit_code':r.returncode,'benchmark':json.loads(r.stdout) if r.stdout.strip() else None,'resources':json.loads(resource[-1]) if resource else None,'stderr':r.stderr}
  log.write(json.dumps(data)+'\n');log.flush();print(json.dumps(data),flush=True)
  if r.returncode:raise RuntimeError(r.stderr)
