import subprocess,pathlib,json,re,os
from html.parser import HTMLParser
class Links(HTMLParser):
 def __init__(self):super().__init__();self.links=[];self.images=[]
 def handle_starttag(self,t,a):
  d=dict(a)
  if t=='a':self.links.append(d.get('href',''))
  if t=='img':self.images.append((d.get('src',''),d.get('alt','')))
p=pathlib.Path(__file__).resolve().parent;out=p/'results';out.mkdir(exist_ok=True)
results=[]
for name in ['small','medium','large','features','inline','inline-important','invalid','void-close','malformed']:
 row={'template':name,'engines':{}}
 for engine in ['mjml','mrml']:
  cmd=['node','bench.cjs'] if engine=='mjml' else ['mrml-bench']
  r=subprocess.run(['docker','run','--rm','--network=none','--memory=1g','-e','CHECK_ONLY=1','-v',str(out)+':/out','-v',str(p/'templates')+':/bench/templates:ro',os.environ.get('BENCHMARK_IMAGE','mjml-mrml-bench:warm-render')]+cmd+['templates/'+name+'.mjml','/out/'+name+'-'+engine+'.html'],capture_output=True,text=True)
  row['engines'][engine]={'exit':r.returncode,'stderr':r.stderr}
 if all(x['exit']==0 for x in row['engines'].values()):
  a=(out/(name+'-mjml.html')).read_text();b=(out/(name+'-mrml.html')).read_text()
  pa,pb=Links(),Links();pa.feed(a);pb.feed(b)
  row['same_links']=sorted(pa.links)==sorted(pb.links);row['same_images']=sorted(pa.images)==sorted(pb.images)
  row.update(bytes_mjml=len(a.encode()),bytes_mrml=len(b.encode()),identical=a==b)
  tokens=set(re.findall(r'(?:FEATURE_[A-Z_]+|INLINE_PROBE|BENCHMARK_USER)',(p/'templates'/(name+'.mjml')).read_text()))
  row['markers_preserved']=all(t in a and t in b for t in tokens);row['markers']=sorted(tokens)
  if name.startswith('inline'):
   class Styles(HTMLParser):
    def __init__(self):super().__init__();self.matches=[]
    def handle_starttag(self,tag,attrs):
     d=dict(attrs)
     if 'letter-spacing' in d.get('style',''):self.matches.append({'tag':tag,'style':d['style']})
   row['inline_styles']={}
   for e,h in [('mjml',a),('mrml',b)]:
    parser=Styles();parser.feed(h);row['inline_styles'][e]=parser.matches
 results.append(row)
(out/'compatibility.json').write_text(json.dumps(results,indent=2));print(json.dumps(results,indent=2))
