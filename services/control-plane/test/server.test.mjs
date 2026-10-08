import test from 'node:test';
import assert from 'node:assert/strict';
import http from 'node:http';
import { createDevServer } from '../src/server.mjs';
async function withServer(fn){const s=createDevServer();await new Promise((r,j)=>{s.once('error',j);s.listen(0,'127.0.0.1',r);});try{await fn(s.address().port);}finally{await new Promise(r=>s.close(r));}}
function request(port,path='/',options={}){return new Promise((resolve,reject)=>{const r=http.request({host:'127.0.0.1',port,path,...options},s=>{let b='';s.on('data',c=>b+=c);s.on('end',()=>resolve({status:s.statusCode,headers:s.headers,body:JSON.parse(b)}));});r.on('error',reject);r.end();});}
test('health identifies simulation',()=>withServer(async p=>{const r=await request(p,'/healthz');assert.equal(r.status,200);assert.equal(r.body.mode,'simulation');assert.equal(r.body.externalActionsEnabled,false);}));
test('Effect service returns review-pending snapshot',()=>withServer(async p=>{const r=await request(p,'/v1/demo/snapshot');assert.equal(r.status,200);assert.equal(r.body.taskState,'awaiting_review');assert.equal(r.body.externalActionsPerformed,0);}));
test('mutations are disabled',()=>withServer(async p=>assert.equal((await request(p,'/v1/demo/snapshot',{method:'POST'})).status,405)));
test('cross-origin requests rejected',()=>withServer(async p=>assert.equal((await request(p,'/healthz',{headers:{origin:'https://untrusted.invalid'}})).status,403)));
test('unexpected Host rejected',()=>withServer(async p=>assert.equal((await request(p,'/healthz',{headers:{host:'untrusted.invalid'}})).status,403)));
test('unknown routes rejected',()=>withServer(async p=>assert.equal((await request(p,'/missing')).status,404)));
test('no caching or CORS headers',()=>withServer(async p=>{const r=await request(p,'/healthz');assert.equal(r.headers['cache-control'],'no-store');assert.equal(r.headers['access-control-allow-origin'],undefined);}));
